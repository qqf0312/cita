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
//! One of CITA's core components that processing blocks and transaction storage,
//! provides queries, caches query records, and more.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     | Queue   | PubModule   | Message Type     |
//!     | ------- | ----------- | ---------------- |
//!     | chain   | Net         | SyncResponse     |
//!     | chain   | Net         | SyncRequest      |
//!     | chain   | Consensus   | BlockWithProof   |
//!     | chain   | Jsonrpc     | Request          |
//!     | chain   | Auth        | BlockTxHashesReq |
//!     | chain   | Executor    | ExecutedResult   |
//!     | chain   | Snapshot    | SnapshotReq      |
//!     | chain   | Executor    | StateSignal      |
//!
//! 2. Publish channel
//!
//!     | Queue | PubModule | SubModule     | Message Type  |
//!     | ----- | --------- | ------------- | ------------- |
//!     | chain | Chain     | Auth          | BlockTxHashes |
//!     | chain | Chain     | Net           | Status        |
//!     | chain | Chain     | Executor      | Request       |
//!     | chain | Chain     | Executor      | StateSignal   |
//!     | chain | Chain     | Jsonrpc       | Response      |
//!     | chain | Chain     | Net           | SyncResponse  |
//!     | chain | Chain     | Snapshot      | SnapshotResp  |
//!     | chain | Chain     | Executor      | LocalSync     |
//!     | chain | Chain     | Consensus     | RichStatus    |
//!     | chain | Chain     | Executor      | RichStatus    |
//!
//! ### Key behavior
//!
//! the key struct:
//!
//! - [`Chain`]
//! - `Forward`: `forward::Forward`
//! - `BlockProcessor`: `block_processor::BlockProcessor`
//!
//! Construct a caching mechanism with `RowLock<Vec<.. >>` or `RowLock<HashMap<.. >>` and clean it regularly.
//!
//! `Forward` listen to the message bus, handle read commands or forward write commands according to message key.
//!
//! `BlockProcessor` processing according to the forwarded information.
//!
//! [`Chain`]: ../core/libchain/chain/struct.Chain.html
//!
extern crate common_types as types;
extern crate rlp_derive;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate cita_logger as logger;
#[macro_use]
extern crate util;
// #[macro_use]
// extern crate cita_relayer_parser;

extern crate db as cita_db;
extern crate stopwatch;

// extern crate serde_derive;
// extern crate serde;
extern crate serde_json;

mod block_processor;
mod forward;
mod block_encoder;
mod ffi_wrapper;
mod datablock;
mod ec_net_msg;
mod serializedblock;
mod block_serialize;

// use serializedblock::SerializedBlock;
// use ffi_wrapper::{encoder_clothes_w, decoder_clothes_w};
// use datablock::DataBlock;
// use libproto::TryInto;
// use libproto::TryFrom;
// use libproto::blockchain::Block as ProtoBlock;
// use jsonrpc_types::rpc_types::RpcBlock;

use crate::block_processor::BlockProcessor;
use crate::block_serialize::BlockSerialize;
use  crate::block_encoder::BlockEncoder;
use crate::cita_db::kvdb::{Database, DatabaseConfig};
use crate::forward::Forward;
use cita_directories::DataPath;
use clap::App;
use core::db;
use core::libchain;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::{channel, start_pubsub};
use std::sync::Arc;
use std::thread;
use std::time;
use std::time::{Duration};
use stopwatch::{Stopwatch};
use util::set_panic_handler;
use std::fs::File;
// use std::io::prelude::*;
use cita_types::{clean_0x, Address};
use std::str::FromStr;
use std::io::Read;
use util::init::parse_config_from_buffer;
use types::mydb;
use block_encoder::NetConfig;
use block_encoder::PeerConfig;
use std::collections::HashMap;
// use cita_types::H160;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    let matches = App::new("chain")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a chain config file'
                          -s, --stdout 'Log to console'",
        )
        .get_matches();

    let stdout = matches.is_present("stdout");
    micro_service_init!("cita-chain", "CITA:chain", stdout);
    info!("Version: {}", get_build_info_str(true));

    let config_path = matches.value_of("config").unwrap_or("chain.toml");
    let network_path = matches.value_of("network").unwrap_or("network.toml");

    //who am i?
    let path = "address";
    let mut buffer = String::new();
    let localaddr = match File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)) {
        Ok(_) => Address::from_str(clean_0x(&buffer)).unwrap(),
        Err(_) => {
            info!("[Config] Cannot find address file, using a random Address instead.");
            Address::random()
        }
    };
    info!("local_address = {}", localaddr);

    let (tx, rx) = channel::unbounded();
    let (ctx_pub, crx_pub) = channel::unbounded();
    start_pubsub(
        "chain",
        routing_key!([
            Net >> SyncResponse,
            Net >> SyncRequest,
            Consensus >> BlockWithProof,
            Jsonrpc >> Request,
            Auth >> BlockTxHashesReq,
            Executor >> ExecutedResult,
            Executor >> StateSignal,
            Snapshot >> SnapshotReq,
            Synchronizer >> RawBytes,
        ]),
        tx,
        crx_pub,
    );

    // 纠删码request消息
    let (ctx_sub_ec, crx_sub_ec) = channel::unbounded();
    let (_ctx_pub_ec, crx_pub_ec) = channel::unbounded();
    start_pubsub(
        "chain_ec",
        routing_key!([Executor >> RawBytes]),
        ctx_sub_ec,
        crx_pub_ec,
    );


    let nosql_path = DataPath::nosql_path();
    info!("nosql_path is {:?}", nosql_path);
    let db_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&db_config, &nosql_path).unwrap();

    // 数据库
    let mynosql_path = DataPath::nosql_path() + "/mynosql";
    info!("mynosql_path is {:?}", mynosql_path);
    let mydb_config = DatabaseConfig::with_columns(mydb::MYNUM_COLUMNS);
    let mydb = Database::open(&mydb_config, &mynosql_path).unwrap();

    let section_size:u64 = 20;  //编码区块区间的大小
    let w = 8;

    let mut buffer = String::new();
    File::open(network_path)
            .and_then(|mut f| f.read_to_string(&mut buffer))
            .unwrap_or_else(|err| panic!("Error while loading config: [{}]", err));
    let _netconf =  parse_config_from_buffer::<NetConfig>(&buffer)
            .unwrap_or_else(|err| panic!("Error while parsing config: [{}]", err));

    // 所有节点的ip:port
    let path = "../template/nodes.list";
    let mut buffer = String::new();
    let peers = match File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)) {
        Ok(_) => {
            let peerstrvec: Vec<&str> = buffer.split('\n').collect();
            let mut peervec: Vec<PeerConfig> = Vec::new();
            for peer in peerstrvec {
                if peer.len() <= 0 {
                    continue;
                }
                let strpeer = String::from_str(peer).unwrap();
                let strpeer: Vec<&str> = strpeer.split(':').collect();
                let peerconf = PeerConfig::new(String::from_str(strpeer.get(0).unwrap()).unwrap(), String::from_str(strpeer.get(1).unwrap()).unwrap());
                peervec.push(peerconf);
            }
            info!("peer vec {:?}", peervec);
            peervec
        }
        Err(_) => {
            info!("[Config] Cannot find address file, using a random Address instead.");
            let mut peervec: Vec<PeerConfig> = Vec::new();
            peervec.push(PeerConfig::new("0.0.0.0".to_string(), "1337".to_string()));
            peervec
        }
    };

    // 有权限节点的Address
    let path = "../template/authorities.list";
    let mut buffer = String::new();
    let mut allnodes: Vec<Address> = Vec::new();
    let addr_peer = match File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)) {
        Ok(_) => {
            let addrstrvec: Vec<&str> = buffer.split('\n').collect();
            // info!("addrstrvec={:?}", addrstrvec);
            let mut addrvec: HashMap<Address, PeerConfig> = HashMap::new();
            let mut i:usize = 0;
            for addr in addrstrvec {
                if addr.len() <= 0 {
                    continue;
                }
                allnodes.push(Address::from_str(clean_0x(addr)).unwrap());
                addrvec.insert(Address::from_str(clean_0x(addr)).unwrap(), peers.get(i).unwrap().clone());    
                i += 1;
            }
            addrvec
        }
        Err(_) => {
            info!("[Config] Cannot find address file, using a random Address instead.");
            let mut addrvec: HashMap<Address, PeerConfig> = HashMap::new();
            addrvec.insert(Address::random(), peers.get(0).unwrap().clone());
            addrvec
        }
    };

    let chain_config = libchain::chain::Config::new(config_path);
    // let blockencoder_config = block_encoder::Config::new(config_path);
    let chain = Arc::new(libchain::chain::Chain::init_chain(
        Arc::new(db),
        &chain_config,
        section_size,
        allnodes.clone(),
    ));

    let block_encoder = Arc::new(BlockEncoder::init_blockencoder(
        Arc::new(mydb),
        // &blockencoder_config,
        section_size,
        // k, m, 
        addr_peer.len() as u64,
        w,
        _netconf.port.unwrap(),
        localaddr,
        allnodes,
        ctx_pub.clone(),
        crx_sub_ec.clone(),
        // addr_peer,
    ));
    block_encoder.set_addr_peer(addr_peer);
    // block_encoder.set_allnodes(allnodes);

    //无限容量的缓冲区 ReceiverFlavor::List
    let (write_sender, write_receiver) = channel::unbounded();
    let forward = Forward::new(Arc::clone(&chain), ctx_pub.clone(), write_sender, Arc::clone(&block_encoder));

    let block_processor = BlockProcessor::new(Arc::clone(&chain), ctx_pub);
    let block_serialize = BlockSerialize::new(Arc::clone(&chain), Arc::clone(&block_encoder));

    // Two threads, one for reading, one for writing
    // Read: dispatch msg
    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            forward.dispatch_msg(&key, &msg);
        }
    });

    // Write: add block
    thread::spawn(move || {
        let mut timeout_factor = 0u8;
        loop {
            // 等待从通道接收消息，但仅在有限的时间内。
            if let Ok(einfo) = write_receiver
                .recv_timeout(Duration::new(18 * (2u64.pow(u32::from(timeout_factor))), 0))
            {
                // info!("new block{}", 1);
                let sw = Stopwatch::start_new();
                block_processor.set_executed_result(&einfo);
                info!("Sava a block took {:?} ms",sw.elapsed_ms());
                timeout_factor = 0;
            } else if !*block_processor.chain.is_snapshot.read() {
                // Here will be these status:
                // 1. Executor process restarts, lost cached block information.
                // 2. Executor encountered an invalid block and cleared the block map.
                // 3. Bft restarted, lost chain status information, unable to consensus, unable to generate block.
                //
                // This will trigger:
                // 1. Network retransmits block information or initiates a synchronization request,
                //    and then the executor will receive a block message
                // 2. Bft will receive the latest status of chain
                info!("Chain enters the timeout retransmission phase");
                block_processor.reset_max_store_height();
                block_processor.signal_to_executor();
                block_processor.broadcast_current_status();
                if timeout_factor < 6 {
                    timeout_factor += 1
                }
            }
        }
    });

    // 纠删码线程
    let sleeptime = 3000;// * section_size; //等时间，每3s出一个块
    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_millis(sleeptime)); //等一会    
        // let oldcurrent = block_serialize.get_old_height(); //上一次编码后的高度
        // let newcurrent = block_serialize.get_chain_height(); //目前区块高度
        // info!("newcurrent = {:?}", newcurrent);
        // let _block = block_serialize.getblock(newcurrent).unwrap();

        // info!("oldcurrent={}, newcurrent={}", oldcurrent, newcurrent);
        // //如果 新增的区块高大于区间大小section_size
        // if newcurrent - oldcurrent > section_size {
            // let start = time::now();
            let sw = Stopwatch::start_new();
            // let value = fib(n);
            //     let sy_time = SystemTime::now();
            block_serialize.encoder();
            // let end = time::now();
            // info!("done! duration: {:?}", SystemTime::now().duration_since(sy_time).unwrap().as_secs());

            info!("It took {:?} ms",sw.elapsed_ms());
        // }
    });


    //garbage collect
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        if chain.cache_size().total() > chain_config.cache_size.unwrap() / 2 {
            info!("cache_manager begin collect garbage...");
            chain.collect_garbage();
        }
    }
}
