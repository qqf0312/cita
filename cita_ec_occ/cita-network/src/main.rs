// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! ## Summary
//!
//! One of the CITA's core components is used to implement the peer-to-peer network  CITA的核心组件之一用于实现对等网络
//! and provide point-to-point connection interaction.  并提供点对点连接交互
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel  订阅
//!
//!     |       Queue       | PubModule | Message Type          |
//!     | ----------------- | --------- | --------------------- |
//!     | network_tx        | Auth      | Request               |
//!     | network_consensus | Consensus | CompactSignedProposal |
//!     | network_consensus | Consensus | RawBytes              |
//!     | network           | Chain     | Status                |
//!     | network           | Chain     | SyncResponse          |
//!     | network           | Jsonrpc   | RequestNet            |
//!     | network           | Jsonrpc   | RequestPeersInfo      |
//!     | network           | Auth      | GetBlockTxn           |
//!     | network           | Auth      | BlockTxn              |
//!
//! 2. Publish channel  发布
//!
//!     |       Queue       | PubModule | SubModule           | Message Type          |
//!     | ----------------- | --------- | ------------------- | --------------------- |
//!     | network           | Net       | Chain, Executor     | SyncResponse          |
//!     | network           | Net       | Snapshot            | SnapshotResp          |
//!     | network           | Net       | Jsonrpc             | Response              |
//!     | network_tx        | Net       | Auth                | Request               |
//!     | network_consensus | Net       | Consensus           | ComapctSignedProposal |
//!     | network_consensus | Net       | Consensus           | RawBytes              |
//!     | network           | Net       | Auth                | BlockTxn              |
//!     | network           | Net       | Auth                | GetBlockTxn           |
//!
//! ### p2p binary protocol  p2p(对等peer to peer)二进制协议
//! | Start      | Full length | Key length | Key value      | Message value    |
//! | ---------- | ----------- | ---------- | -------------- | ---------------- |
//! | \xDEADBEEF | u32         | u8(byte)   | bytes of a str | a serialize data |
//!
//! full_len = 1 + key_len + body_len
//!
//! ### Key behavoir
//!
//! the key struct:
//!
//! - [`Connection`]
//! - [`NetWork`]
//! - [`Synchronizer`]  同步器
//!
//! In addition to the `tokio_server`, there is an `Arc<Connection>` for
//! this structure in almost all the threads of this module to confirm that the node is alive,
//! increase or decrease nodes, consensus message broadcasts, authentication message broadcasts,
//! node status broadcasts, synchronization node blocks Height and so on.
//!
//! About binary protocol encoding and decoding, please look at module `citaprotocol`, the fuction
//! [`pubsub_message_to_network_message`] and [`network_message_to_pubsub_message`].
//!
//! [`Connection`]: ./connection/struct.Connection.html
//! [`NetWork`]: ./network/struct.NetWork.html
//! [`Synchronizer`]: ./synchronizer/struct.Synchronizer.html
//! [`pubsub_message_to_network_message`]: ./citaprotocol/fn.pubsub_message_to_network_message.html
//! [`network_message_to_pubsub_message`]: ./citaprotocol/fn.network_message_to_pubsub_message.html
//!

#[macro_use]
extern crate cita_logger as logger;

pub mod cita_protocol;
pub mod config;
pub mod mq_agent;
pub mod network;
pub mod node_manager;
pub mod p2p_protocol;
pub mod synchronizer;

use crate::config::{AddressConfig, NetConfig};
use crate::mq_agent::MqAgent;
use crate::network::Network;
use crate::node_manager::{NodesManager, DEFAULT_PORT};
use crate::p2p_protocol::{
    node_discovery::create_discovery_meta, transfer::create_transfer_meta, SHandle,
};
use crate::synchronizer::Synchronizer;
use clap::App;
use dotenv;
use futures::prelude::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use tentacle::{builder::ServiceBuilder, secio::SecioKeyPair};
use util::micro_service_init;
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

//延时
const NOTIFY_DELAY_SECS: u64 = 1;

fn main() {
    // init app
    let matches = App::new("network")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
                        -a, --address=[FILE] 'Sets an address file'
                        -s, --stdout 'Log to console'",
        )
        .get_matches();

    let stdout = matches.is_present("stdout");
    micro_service_init!("cita-network", "CITA:network", stdout);
    info!("Version: {}", get_build_info_str(true));

    //value_of 获取特定选项或位置参数的值（即，在运行时需要附加值的参数）。
    // 如果该选项在运行时不存在，则返回None。
    let config_file = matches.value_of("config").unwrap_or("network.toml");

    let config_path = Path::new(config_file);
    let mut dir = config_path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_str()
        .unwrap();
    if dir.is_empty() {
        dir = ".";
    }
    let fname = config_path
        .file_name()
        .expect("Wrong config file")
        .to_str()
        .unwrap()
        .to_string()
        .clone();

    // Init config
    debug!("Config path {:?}", config_path);
    let config = NetConfig::new(&config_file);
    // debug!("Network config is {:?}", config_file);
    info!("Network config is {:?}", config);

    let addr_path = matches.value_of("address").unwrap_or("address");
    let own_addr = AddressConfig::new(&addr_path);
    // debug!("Node address is {:?}", own_addr.addr);
    info!("Node address is {:?}", own_addr);
    // End init config

    // Init p2p protocols  p2p协议
    let mut nodes_mgr = NodesManager::from_config(config.clone(), own_addr.addr);
    // 从MQ订阅了“ Auth”请求
    let mut mq_agent = MqAgent::default();
    // Synchronizer 获取消息并确定是否需要同步或广播当前节点状态
    let mut synchronizer_mgr = Synchronizer::new(mq_agent.client(), nodes_mgr.client());
    let mut network_mgr = Network::new(
        mq_agent.client(),
        nodes_mgr.client(),
        synchronizer_mgr.client(),
    );
    mq_agent.set_nodes_mgr_client(nodes_mgr.client());
    mq_agent.set_network_client(network_mgr.client());

    let transfer_meta =
        create_transfer_meta(network_mgr.client(), nodes_mgr.client(), own_addr.addr);
        //创建服务
        let mut service_cfg = ServiceBuilder::default()
        .insert_protocol(transfer_meta)
        .forever(true);

    let discovery_flag = config.enable_discovery.unwrap_or(true);
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, std::time::Duration::from_secs(NOTIFY_DELAY_SECS)).unwrap();
    if discovery_flag { // 能自动发现节点
        let discovery_meta = create_discovery_meta(nodes_mgr.client());
        service_cfg = service_cfg.insert_protocol(discovery_meta);  //插入自定义协议
    } else if watcher.watch(dir, RecursiveMode::NonRecursive).is_ok() { //不能 则监测配置文件的变化
        let notify_client = nodes_mgr.client();
        thread::spawn(move || {
            NodesManager::notify_config_change(rx, notify_client, fname);
        });
    }

    if config.enable_tls.unwrap_or(false) {
        service_cfg = service_cfg.key_pair(SecioKeyPair::secp256k1_generated());
    }
    let mut service = service_cfg.build(SHandle::new(nodes_mgr.client()));

    let addr = format!("/ip4/0.0.0.0/tcp/{}", config.port.unwrap_or(DEFAULT_PORT));
    let _ = service.listen(addr.parse().unwrap());
    nodes_mgr.set_service_task_sender(service.control().clone());
    // End init p2p protocols

    // Run system
    mq_agent.run();  //监听
    thread::spawn(move || nodes_mgr.run()); // 节点管理模块
    thread::spawn(move || network_mgr.run()); // 网络服务模块
    thread::spawn(move || synchronizer_mgr.run()); //同步模块
    tokio::run(service.for_each(|_| Ok(())));
    // End run system
}
