// #[macro_use]
// extern crate cita_relayer_parser;
// use core::basic_types::LogBloom;
// use core::bloomchain::group::BloomGroup;
// use core::bloomchain::group::BloomGroupChain;
// use core::bloomchain::group::BloomGroupDatabase;
// use core::bloomchain::group::GroupPosition as BloomGroupPosition;
// use core::bloomchain::Bloom;
// use core::bloomchain::Config as BloomChainConfig;
// use core::bloomchain::Number as BloomChainNumber;
use crate::mydb;
use crate::mydb::*;

// use crate::db;
// use crate::db::*;

// use cita_directories::DataPath;
// pub use byteorder::{BigEndian, ByteOrder};

// use crate::filters::{PollFilter, PollManager};
// use crate::header::*;
// use crate::libchain::cache::CacheSize;
// use crate::libchain::status::Status;
// pub use crate::types::block::*;
use crate::types::extras::*;
// use libproto::blockchain::{
//     AccountGasLimit as ProtoAccountGasLimit, Proof as ProtoProof, ProofType,
//     RichStatus as ProtoRichStatus, StateSignal,
// };
use libproto::OperateType;

use crate::cita_db::kvdb::*;
use core::header::Header;
// use crate::receipt::{LocalizedReceipt, Receipt};
// use crate::types::cache_manager::CacheManager;
// use crate::types::filter::Filter;
// use crate::types::ids::{BlockId, TransactionId};
use crate::types::log_entry::LocalizedLogEntry;
// use crate::types::log_entry::LogEntry;
use crate::types::transaction::Action;
use crate::types::transaction::SignedTransaction;
use cita_merklehash;
// use cita_types::traits::LowerHex;
// use cita_types::{Address, H256, U256};
use hashable::Hashable;
// use libproto::executor::ExecutedResult;
use libproto::router::{MsgType, RoutingKey, SubModules};
// use libproto::TryInto;
// use libproto::BlockTxHashes;
use libproto::FullTransaction;
// use libproto::Message;
// use proof::BftProof;
// use pubsub::channel::Sender;
// use rlp::{self, Encodable};
// use std::collections::BTreeMap;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::convert::Into;
// use std::sync::atomic::AtomicBool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
// use util::HeapSizeOf;
// use util::Mutex;
// use util::RwLock;
use crate::ffi_wrapper::decoder_clothes_w;
use crate::types::ids::BlockId;
use crate::types::receipt::LocalizedReceipt;
use common_types::BlockNumber;
// use jsonrpc_types::rpc_types::BlockNumber as RpcBlockNumber;
use common_types::block::Block;
use common_types::block::BlockBody;
// use common_types::block::OpenBlock;
// use libproto::blockchain::Block as ProtoBlock;
use cita_types::H256;
use cita_types::H160;
use crate::datablock::DataBlock;
use crate::ec_net_msg::EcNetMessage;
// use libproto::TryInto;
// use libproto::TryFrom;
use std::collections::HashMap;
use rlp::*;
use crate::types::ids::TransactionId;
use cita_types::Address;
// use crate::types::cache_manager::CacheManager;

use serde::Serialize;
use serde::Deserialize;

// use serde::{Deserialize, Serialize};
// use std::error::Error;
// use reqwest;

// use jsonrpc_types::rpc_request;
// use jsonrpc_types::rpc_response;
use jsonrpc_types::rpc_types;
// use std::time::Duration;
// use cita_types::U256;
// use cita_relayer_parser::communication::RpcClient;
// use cita_relayer_parser::communication::Error;
// use cita_relayer_parser::configuration::UpStream;

// use std::convert::From;
// use std::option::NoneError;

// use hyper;
// use futures::future::Either;
// use futures::sync::{mpsc, oneshot};
// use futures::{Future, Sink, Stream};
// use tokio_core::reactor::{Core, Timeout};
// use parking_lot::Mutex;
use parking_lot::RwLock;
use types::extras::ConsensusNodes;
// use libproto::response::Response;

// pub mod cita_relayer_parser;


// use futures::future::Either;
// use futures::sync::{mpsc, oneshot};
// use futures::{Future, Sink, Stream};
// use hyper;
use libproto::TryInto;
// use parking_lot::{Mutex, RwLock};
use serde_json;
use std::convert::Into;
use jsonrpc_types::rpc_types::Block as JsonBlock;
// use tokio_core::reactor::{Core, Timeout};

// use cita_types::{H256, U256};
// use jsonrpc_types::{rpc_request, rpc_types};
// use libproto::blockchain::UnverifiedTransaction;


// use cita_relayer_parser::communication;

// const LOG_BLOOMS_LEVELS: usize = 3;
// const LOG_BLOOMS_ELEMENTS_PER_INDEX: usize = 16;
use pubsub::channel::Sender;
use pubsub::channel::Receiver;
// use libproto::RawBytes;
// use crate::serializedblock::SerializedBlock;
// use std::thread;
use libproto::{Message, TryFrom};

// comunication.rs

// #[derive(Debug)]
// pub enum Error {
//     BadStatus,
//     Timeout,
//     Parse,
// }

// impl From<NoneError> for Error {
//     fn from(err: NoneError) -> Self {
//         Error::BadStatus
//     }
// }

// type RpcSender =
//     Mutex<mpsc::Sender<(hyper::Request, oneshot::Sender<Result<hyper::Chunk, Error>>)>>;

// pub struct RpcClient {
//     sender: RpcSender,
//     uri: RwLock<hyper::Uri>,
// }

// impl RpcClient {
//     pub fn create(upstream: &UpStream) -> ::std::sync::Arc<Self> {
//         // 创建线程
//         let tb = ::std::thread::Builder::new().name("RpcClient".to_string());
//         // 将url转变成uri
//         let uri = upstream.url.parse::<hyper::Uri>().unwrap();
//         // 发送 接收服务定义
//         let (tx, rx) =
//             mpsc::channel::<(hyper::Request, oneshot::Sender<Result<hyper::Chunk, Error>>)>(65_535);
//         // 可延迟时间
//         let timeout_duration = upstream.timeout;

//         // 开始一个线程
//         let _tb = tb
//             .spawn(move || {
//                 let mut core = Core::new().unwrap();
//                 let handle = core.handle();
//                 // 客户端
//                 let client = hyper::Client::configure()
//                     .connector(hyper::client::HttpConnector::new(4, &handle))
//                     .keep_alive(false)
//                     .build(&handle);

//                 let messages = rx.for_each(|(req, sender)| {
//                     let timeout = Timeout::new(timeout_duration, &handle).unwrap();
//                     let post = client.request(req).and_then(|res| res.body().concat2());

//                     let work = post.select2(timeout).then(move |res| match res {
//                         Ok(Either::A((got, _timeout))) => {
//                             // info!("sender.send(Ok(got)){:?}", got);
//                             let _ = sender.send(Ok(got));
//                             Ok(())
//                         }
//                         Ok(Either::B(_)) | Err(_) => {
//                             // info!("sender.send(Err(Error::Timeout)){:?}", Error::Timeout);
//                             let _ = sender.send(Err(Error::Timeout));
                            
//                             Ok(())
//                         }
//                     });

//                     handle.spawn(work);
//                     Ok(())
//                 });

//                 core.run(messages).unwrap();
//             })
//             .expect("Couldn't spawn a thread.");

//         ::std::sync::Arc::new(RpcClient {
//             sender: Mutex::new(tx),
//             uri: RwLock::new(uri),
//         })
//     }

//     pub fn do_post(&self, body: &str) -> Result<hyper::Chunk, Error> {
//         let uri = { self.uri.read().clone() };

//         // let o = Post {
//         //     user_id: 1,
//         //     id: 0,                
//         //     title: "test title".to_string(),
//         //     body: "test body".to_string()
//         // };
        
//         // info!("Send body {:?} to {:?}.", body, uri);
//         // let body = serde_json::to_string(&o).unwrap();
//         info!("Send body {:?} to {:?}.", body, uri);
        
//         let mut req = hyper::Request::new(hyper::Method::Post, uri);
//         req.headers_mut().set(hyper::header::ContentType::json());
//         req.set_body(body.to_owned());
//         // req.set_body(body);
//         let (tx, rx) = oneshot::channel();
//         {
//             let _ = self.sender.lock().start_send((req, tx));
//         }
//         match rx.wait() {
//             Ok(res) => {
//                 match res {
//                     Ok(chunk) => {
//                         // info!("Get response {:?}.", chunk);
//                         Ok(chunk)
//                     }
//                     Err(_) => {
//                         Err(Error::Timeout)
//                     }
//                 }
//             }
//             Err(_) => Err(Error::BadStatus),
//         }
//     }
// }

// #[derive(Debug, Hash, Eq, PartialEq, Clone)]
// pub enum MyCacheId {
//     DataBlocks(BlockNumber),
//     BlockHashes(H256),
//     TransactionAddresses(H256),
//     BlocksBlooms(LogGroupPosition),
//     BlockReceipts(H256),
// }

// #[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
// pub struct Config {
//     pub prooftype: u8,
//     pub cache_size: Option<usize>,
// }

// impl Config {
//     pub fn default() -> Self {
//         Config {
//             prooftype: 2,
//             cache_size: Some(1 << 20),
//         }
//     }

//     pub fn new(path: &str) -> Self {
//         let mut c: Config = parse_config!(Config, path);
//         if c.cache_size.is_none() {
//             c.cache_size = Some(1 << 20 as usize);
//         }
//         c
//     }
// }

// impl BloomGroupDatabase for BlockEncoder {
//     fn blooms_at(&self, position: &BloomGroupPosition) -> Option<BloomGroup> {
//         let position = LogGroupPosition::from(position.clone());
//         let result = self
//             .mydb
//             .read_with_cache(mydb::MYCOL_EXTRA, &self.blocks_blooms, &position)
//             .map(Into::into);
//         self.cache_man
//             .lock()
//             .note_used(MyCacheId::BlocksBlooms(position));
//         result
//     }
// }

/// Processing blocks and transaction storage
pub struct BlockEncoder {
    // pub blooms_config: BloomChainConfig,
    pub section_size:u64,  //编码区块区间的大小
    pub oldn:AtomicUsize, //
    pub kmap:RwLock<HashMap<u64, u64>>, //<sectionid, k>
    pub mmap:RwLock<HashMap<u64, u64>>, //<sectionid, m>
    pub w:u64,
    // 本节点端口号
    pub port: usize,
    // 本节点地址
    pub address: Address,
    //网络配置
    pub addr_peer:RwLock<HashMap<H160, PeerConfig>>,
    // 全部节点的网络配置
    pub allnodes: RwLock<ConsensusNodes>,

    // 区间id
    pub sectionid:AtomicUsize,
    //上一次编码的区块高度
    pub old_height:AtomicUsize,

    // datablock cache
    //数据+冗余块数组
    pub datablockmap:RwLock<HashMap<String, Vec<u8>>>,
    //数据块大小数组
    pub datablocksizemap: RwLock<HashMap<String, usize>>,
    // 本区间数据块大小的最大值
    pub maxdatablocksizemap: RwLock<HashMap<u64, usize>>, //<sectionid, maxxdatablocksize>
    
    // 参与数据块ec的节点地址(共识节点地址)
    pub block_nodes_map: RwLock<HashMap<u64, ConsensusNodes>>,

    // extra cache
    // hash与height的map映射
    pub hash_height_map: RwLock<HashMap<H256, BlockNumber>>,
    // 交易id与交易地址的map映射
    pub transaction_addresses: RwLock<HashMap<TransactionId, TransactionAddress>>,


    // pub blocks_blooms: RwLock<HashMap<LogGroupPosition, LogBloomGroup>>,
    // 交易回执映射
    pub block_receipts: RwLock<HashMap<H256, BlockReceipts>>,
    // 数据库
    pub mydb: Arc<KeyValueDB>,

    // 发送消息的通道
    ctx_pub: Sender<(String, Vec<u8>)>,

    crx_sub_ec: Receiver<(String, Vec<u8>)>,
    // 
    // pub cache_man: Mutex<CacheManager<MyCacheId>>,

    // pub block_receipts: RwLock<HashMap<H256, BlockReceipts>>,
}

/// Get latest status
// pub fn get_block_encoder(db: &KeyValueDB) -> Option<Header> {
//     // CANNOT replace CurrentHash & hash with CurrentHeight to get current_height,
//     // because CurrentHeight is set after BlockBody is stored, and CurrentHash is set after BlockHeader is stored.
//     let h: Option<H256> = db.read(mydb::MYCOL_EXTRA, &CurrentHash);
//     if let Some(hash) = h {
//         let hi: Option<BlockNumber> = db.read(mydb::MYCOL_EXTRA, &hash);
//         if let Some(h) = hi {
//             trace!("get_chain hash {:?}  bn{:?}  CurrentHash", hash, h);
//             db.read(mydb::MYCOL_HEADERS, &h)
//         } else {
//             warn!("not expected get_chain_current_head height");
//             None
//         }
//     } else {
//         warn!("not expected get_chain_current_head hash.");
//         None
//     }
// }

pub fn get_encoder_oldheight(db: &KeyValueDB) -> Option<BlockNumber> {
    db.read(mydb::MYCOL_EXTRA, &OldHeight)
}

pub fn get_encoder_sectionid(db: &KeyValueDB) -> Option<BlockNumber> {
    db.read(mydb::MYCOL_EXTRA, &SectionId)
}

pub fn get_encoder_oldn(db: &KeyValueDB) -> Option<BlockNumber> {
    db.read(mydb::MYCOL_EXTRA, &OldN)
}

pub fn get_allnodes(db: &KeyValueDB) -> Option<ConsensusNodes> {
    db.read(mydb::MYCOL_EXTRA, &AllNodes)
}

// pub fn contract_address(address: &Address, nonce: &U256) -> Address {
//     use rlp::RlpStream;
//     let mut stream = RlpStream::new_list(2);
//     stream.append(address);
//     stream.append(nonce);
//     From::from(stream.out().crypt_hash())
// }

impl BlockEncoder {
    pub fn init_blockencoder(
        mydb: Arc<KeyValueDB>, 
        // chain_config: &Config,
        section_size:u64, 
        // k: u64, m: u64, 
        oldn:u64,
        w: u64,
        port: usize, 
        address: Address,
        allnodes: Vec<Address>,
        ctx_pub: Sender<(String, Vec<u8>)>,
        crx_sub_ec: Receiver<(String, Vec<u8>)>,
        // addr_peer: HashMap<H160, PeerConfig>,
    ) -> BlockEncoder {

        // let old_height = AtomicUsize::new(0);

        let old_height = AtomicUsize::new(0);
        if let Some(height) = get_encoder_oldheight(&*mydb) {
            info!("old height: {:?}", height);
            old_height.store(height as usize, Ordering::SeqCst);
        }

        let sectionid = AtomicUsize::new(0);
        if let Some(secid) = get_encoder_sectionid(&*mydb) {
            info!("sectionid: {:?}", secid);
            sectionid.store(secid as usize, Ordering::SeqCst);
        }

        let oldn = AtomicUsize::new(oldn as usize);
        if let Some(n) = get_encoder_oldn(&*mydb) {
            info!("oldn: {:?}", oldn);
            oldn.store(n as usize, Ordering::SeqCst);
        }

        let allnodes = RwLock::new(ConsensusNodes::new(allnodes));
        if let Some(nodes) = get_allnodes(&*mydb) {
            allnodes.write().clone_from(&nodes);
            info!("allnodes: {:?}", allnodes.read().nodes);
        } else {
            info!("allnodes: {:?}", allnodes.read().nodes);
        }

        // // 400 is the avarage size of the key
        // let cache_man = CacheManager::new(
        //     chain_config.cache_size.unwrap() * 3 / 4,
        //     chain_config.cache_size.unwrap(),
        //     400,
        // );

        // let blooms_config = BloomChainConfig {
        //     levels: LOG_BLOOMS_LEVELS,
        //     elements_per_index: LOG_BLOOMS_ELEMENTS_PER_INDEX,
        // };

        // let header = get_block_encoder(&*mydb).unwrap_or_default();
        // // debug!("get chain head is : {:?}", header);
        // // info!("get chain head is : {:#?}", header);
        // let current_height = AtomicUsize::new(header.number() as usize);
        // let max_store_height = AtomicUsize::new(0);
        // if let Some(height) = get_chain_body_height(&*mydb) {
        //     max_store_height.store(height as usize, Ordering::SeqCst);
        // }
        
        // info!(
        //     "get chain max_store_height : {:?}  current_height: {:?}",
        //     max_store_height, current_height
        // );

        let blockencoder = BlockEncoder {
            // blooms_config,
            section_size,
            kmap: RwLock::new(HashMap::new()),
            mmap: RwLock::new(HashMap::new()),
            oldn,
            w,
            port,
            address,
            addr_peer:RwLock::new(HashMap::new()),
            allnodes:RwLock::new(ConsensusNodes::new(Vec::new())),
            //区间id
            sectionid,
            //上一次编码的区块高度
            old_height,
            //数据块数组
            datablockmap: RwLock::new(HashMap::new()),
            //数据块大小数组
            datablocksizemap: RwLock::new(HashMap::new()),
            //一个编码区间数据块大小 最大值
            maxdatablocksizemap: RwLock::new(HashMap::new()),
            // 参与数据块ec的节点地址(共识节点地址)
            block_nodes_map: RwLock::new(HashMap::new()),
            // hash与height的map映射
            hash_height_map: RwLock::new(HashMap::new()),
            // 交易id与交易地址的map映射
            transaction_addresses: RwLock::new(HashMap::new()),
        
            block_receipts: RwLock::new(HashMap::new()),
            mydb,
            // cache_man: Mutex::new(cache_man),
            ctx_pub,
            crx_sub_ec,
        };
        blockencoder
    }

    // 存储编码结果
    pub fn set_encoder_result(&self, 
        sectionid:u64, 
        old_height: u64, 
        oldn:u64, 
        k: u64, 
        m: u64,
        datablockid: String, 
        datablock: Vec<u8>,  
        datablocksize: usize, 
        maxdatablocksize: usize, 
        // datablockmap: HashMap<String, Vec<u8>>,
        // datablocksizemap: HashMap<String, usize>,
        blocknodes: ConsensusNodes, 
        map:HashMap<H256, BlockNumber>, 
        addrs: HashMap<TransactionId, TransactionAddress>,
        receipts: HashMap<H256, BlockReceipts>,
        allnodes: ConsensusNodes,) {

        self.set_sectionid(sectionid + 1);
        self.set_old_height(old_height);
        self.set_oldn(oldn);
        // info!("allnodes.nodes = {:?}, self.get_allnodes().nodes = {:?}", allnodes.nodes, self.get_allnodes().nodes);
        // info!("datablocksize = {:?}", datablocksize);
        // info!("maxdatablocksize = {:?}", maxdatablocksize);

        if allnodes.nodes.len() > self.get_allnodes().nodes.len() {
            self.set_allnodes(allnodes);
        }
            
            self.set_db_result(sectionid.clone(), 
                old_height.clone(), 
                oldn.clone(),
                k.clone(),
                m.clone(),
                datablockid.clone(), 
                datablock.clone(), 
                datablocksize.clone(), 
                maxdatablocksize.clone(),
                // datablockmap,
                // datablocksizemap,
                blocknodes.clone(), 
                map.clone(), 
                addrs.clone(), 
                receipts.clone(),
            );

        
        // self.datablockmap_insert(datablockid.clone(), datablock);
        // self.datablocksizemap_insert(datablockid.clone(), datablocksize);
        // self.blocknodesmap_insert(format!("{}", sectionid - 1), blocknodes.clone());
        // self.set_hash_height_map(map);
        // self.set_transaction_addresses(addrs);
        // self.set_block_receipts(receipts);
    }

    pub fn set_db_result(&self, 
        sectionid:u64, 
        old_height: u64, 
        oldn:u64,
        k: u64,
        m: u64,
        datablockid: String, 
        datablock: Vec<u8>,  
        datablocksize: usize, 
        maxdatablocksize: usize, 
        // datablockmap: HashMap<String, Vec<u8>>,
        // datablocksizemap: HashMap<String, usize>,
        blocknodes: ConsensusNodes, 
        hashheightmap:HashMap<H256, BlockNumber>, 
        transaction_addresses: HashMap<TransactionId, TransactionAddress>,
        block_receipts:HashMap<H256, BlockReceipts>,
    ) {
        
        // 缓冲
        let mut batch = DBTransaction::new();

        // 写区间id和高度
        // db.read(mydb::MYCOL_EXTRA, &SectionId)
        batch.write(mydb::MYCOL_EXTRA, &SectionId, &(sectionid + 1));
        batch.write(mydb::MYCOL_EXTRA, &OldHeight, &old_height);
        batch.write(mydb::MYCOL_EXTRA, &OldN, &oldn);

        // 写k m
        let mut write_k = self.kmap.write();
        batch.write_with_cache(
            mydb::MYCOL_K, 
            // cache,
            &mut *write_k, 
            sectionid.clone(), 
            k, 
            CacheUpdatePolicy::Overwrite,
        );

        let mut write_m = self.mmap.write();
        batch.write_with_cache(
            mydb::MYCOL_M, 
            // cache,
            &mut *write_m, 
            sectionid.clone(), 
            m, 
            CacheUpdatePolicy::Overwrite,
        );

        // if datablocksize != 0 {
            // 写数据块
            if datablockid != String::from("0_2") {
                let mut write_datablock = self.datablockmap.write();
                // batch.extend_with_cache(
                batch.write_with_cache(
                    mydb::MYCOL_DATABLOCKS, 
                    // cache,
                    &mut *write_datablock, 
                    datablockid.clone(), 
                    datablock, 
                    // datablockmap,
                    CacheUpdatePolicy::Overwrite,
                );
            } else {
                info!("datablockid == 0_2");
                let strdatablock = String::from_utf8(datablock.clone()).unwrap();
                info!("strdatablock={}", strdatablock);
            }
            // 标记被使用 lock
            // self.cache_man.lock().note_used(MyCacheId::DataBlocks(hash));

            // 写数据块大小
            let mut write_datablocksize = self.datablocksizemap.write();
            // batch.extend_with_cache(
            batch.write_with_cache(
                mydb::MYCOL_SIZE, 
                // cache,
                &mut *write_datablocksize, 
                datablockid.clone(), 
                datablocksize,
                // datablocksizemap, 
                CacheUpdatePolicy::Overwrite,
            );
            // 标记被使用

            // 写该区间数据块大小的最大值
            let mut write_maxdatablocksize = self.maxdatablocksizemap.write();
            batch.write_with_cache(
                mydb::MYCOL_MAXSIZE,
                &mut *write_maxdatablocksize,
                sectionid.clone(),
                maxdatablocksize,
                CacheUpdatePolicy::Overwrite,
            );

        // }

        // 写数据块共识节点地址
        let nodes = blocknodes.clone();
        let mut write_datablock = self.block_nodes_map.write();
        batch.write_with_cache(
            mydb::MYCOL_CONSENSUS_NODES, 
            // cache,
            &mut *write_datablock, 
            sectionid, 
            nodes, 
            CacheUpdatePolicy::Overwrite,
        );
        // 标记被使用

        // 写hash height映射
        let mut write_hashes = self.hash_height_map.write();
        batch.extend_with_cache(
            mydb::MYCOL_EXTRA,
            // cache,
            &mut *write_hashes,
            hashheightmap,
            CacheUpdatePolicy::Overwrite,
        );
        // 标记被使用

        // 写交易地址
        let mut write_txs = self.transaction_addresses.write();
            batch.extend_with_cache(
                mydb::MYCOL_EXTRA,
                // cache,
                &mut *write_txs,
                transaction_addresses,
                CacheUpdatePolicy::Overwrite,
            );
        // 标记被使用

        // 写交易回执
        let mut write_receipts = self.block_receipts.write();  //写缓冲区
        batch.extend_with_cache( //db.rs L107
            mydb::MYCOL_EXTRA,
            // cache,
            &mut *write_receipts,  //写缓冲
            // datablockid,
            block_receipts,  //写的内容
            CacheUpdatePolicy::Overwrite,
        );
        // 标记被使用

        self.mydb.write(batch).expect("DB write failed.");
        
    }


    // 获取区块
    pub fn block(&self, id: BlockId) -> Option<Block> {
        // let string = String::from("chain");
        // string.as_mut_vec()
        match id {
            BlockId::Hash(hash) => self.block_by_hash(hash),
            BlockId::Number(number) => self.block_by_height(number),
            BlockId::Earliest => self.block_by_height(0),
            BlockId::Latest => self.block_by_height(self.get_latest_height()),
            BlockId::Pending => self.block_by_height(self.get_pending_height()),
        }
    }

    /// Get block by hash
    pub fn block_by_hash(&self, hash: H256) -> Option<Block> {
        self.block_height_by_hash(hash)
                .and_then(|h| self.block_by_height(h))
    }

    pub fn block_height_by_hash(&self, hash: H256) -> Option<BlockNumber> {
        let result = self
            .mydb
            .read_with_cache(mydb::MYCOL_EXTRA, &self.hash_height_map, &hash);
        // 标记被使用TODO
        // match self.get_hash_height_map().get(&hash) {
        //     Some(h) => {
        //         result = Some(h.clone());
        //     }
        //     None => {
        //         result = None;
        //     }
        // }
        result
    }

    /// Get block by height 响应其他节点发起的请求
    pub fn block_by_height_for_ec(&self, number: BlockNumber) -> Option<Vec<u8>> {
        
        // let m = self.m;
        // let w = self.w;
        let section_size = self.section_size;
        let mut block;
        // 计算对应的数据块id
        // let number = number - 1; // height = number - 1
        info!("block_by_height: height={}", number);
        let sectionid = number / section_size; //能不能被整除一样
        let k = self.get_kmap(sectionid).unwrap().clone();
        info!("k = {}", k);
        let datablockid:u64; // 区间中的第几块
        if section_size % k == 0 {
            datablockid = (number % section_size) / (section_size / k);
        } else {
            if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
                datablockid = (number % section_size) / (section_size / k + 1);
            } else{
                datablockid = section_size % k + (number % section_size - (section_size / k + 1) * (section_size % k)) / (section_size / k);
            }
        }

        info!("sectionid = {}, datablockid = {}", sectionid, datablockid);
        let strdatablockid = format!("{0}_{1}", sectionid, datablockid);
        // 根据数据块id获取数据块
        match self.get_datablockmap(strdatablockid.clone()) {
            Some(datablock) => {
                // 读取一个数据块中的有效信息
                let bufdatablock = datablock.clone();
                // info!("bufdatablock = {:?}", bufdatablock);
                
                // 转换成string格式
                let strdatablock = String::from_utf8(bufdatablock).unwrap();
                info!("strdatablock={}", strdatablock);

                //转换成数据块DataBlock格式
                let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

                // 从数据块中取出序列化的区块
                let blocks = desedatablock.databody;

                let blockid:usize;

                if section_size % k == 0 {
                    blockid = (number % (section_size / k)) as usize;
                } else {
                    if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
                        blockid = ((number % section_size) % (section_size / k + 1)) as usize;
                    } else{
                        blockid = ((number % section_size - (section_size / k + 1) * (section_size % k)) % (section_size / k)) as usize;
                    }
                }

                info!("blockid = {}", blockid);
                let serblock = blocks.get(blockid).unwrap().clone();
                // info!("serblock get ok! blockid = {}, height = {}", blockid, serblock.id);
                // info!("serblock.blockbody = {:?}", serblock.blockbody);
                block = Some(serblock.blockbody.clone());
                // info!("RLP  New Block={:?}", block);
            }
            None => {
                let mut blocku8:Vec<u8> = Vec::new();  //u8类型 一维数据块 传入参数
                let mut i:usize = 0;
                let maxdatablocksize = self.get_maxblocknodesize(sectionid).unwrap();
                let k = self.get_kmap(sectionid).unwrap();
                let m = self.get_mmap(sectionid).unwrap();
                let mut erasures:Vec<i32> = Vec::with_capacity((k + m) as usize); //记录缺失的数据块
                
                let mut lostdatablock:Vec<u8> = Vec::with_capacity(maxdatablocksize); //填补缺失块
                for _i in 0..maxdatablocksize {
                    lostdatablock.push(0x00);
                }
                
                // 获取区块所在节点的序号
                let addresses = self.get_blocknodesmap(sectionid).unwrap().clone();
                info!("all address:{:?}", addresses);
                
                for addr in addresses.nodes {
                    if addr == self.address { //本节点丢失的区块
                        blocku8.append(&mut lostdatablock);   
                        erasures.push(i as i32);
                        info!("1 blocku8.size = {:?}", blocku8.len());
                    } else {
                        match self.de_reqwest_block(addr, format!("{0}_{1}", sectionid, i)) {
                            Some(datablock) => {
                                blocku8.append(&mut datablock.clone());
                                let mut j = datablock.len();
                                while j < maxdatablocksize {
                                    blocku8.push(0x00);
                                    j += 1;
                                }
                                info!("2 blocku8.size = {:?}", blocku8.len());
                            }
                            None => {
                                blocku8.append(&mut lostdatablock.clone());
                                erasures.push(i as i32);
                                info!("3 blocku8.size = {:?}", blocku8.len());
                            }
                        }
                    }
                    i += 1;
                }
                    
                erasures.push(-1);
                info!("erasures = {:?}", erasures);
                info!("maxdatablocksize = {:?}", maxdatablocksize);
                info!("blocku8.size = {:?}", blocku8.len());
                let mut coding = blocku8.split_off(maxdatablocksize * k as usize);
                let praw_coding = coding.as_mut_ptr();
                let strdatablock = String::from_utf8(blocku8.clone()).unwrap();
                info!("strdatablock={:?}", strdatablock);
                // info!("blocku8 = {:?}", blocku8);
                let puncode_stream = blocku8.as_mut_ptr();
                let raw_datablocksize = maxdatablocksize;
                let perasures = erasures.as_mut_ptr();

                unsafe {
                    for _i in 0..erasures.len() {
                            info!("{:?}", *perasures.add(_i));
                        }
                    }
                
                decoder_clothes_w(k, m, self.w, puncode_stream, praw_coding , raw_datablocksize, perasures);

                info!("decoder ok!");
                let mut decodedblocku8 = blocku8.clone();

                // ？？？？？？？？？
                let strdatablock = String::from_utf8(decodedblocku8.clone()).unwrap();
                info!("strdatablock={:?}", strdatablock);

                let strcoding = String::from_utf8(coding.clone()).unwrap();
                info!("strcoding={:?}", strcoding);

                for _i in 0..datablockid {
                    decodedblocku8 = decodedblocku8.split_off(maxdatablocksize);
                }

                // 读取一个数据块中的有效信息
                let datablocksize = self.get_datablocksize(format!("{0}_{1}", sectionid, datablockid)).unwrap().clone(); 
                info!("datablocksize = {}", datablocksize);
                decodedblocku8.split_off(datablocksize);
                info!("decodedblocku8.size = {}", decodedblocku8.len());
                
                // info!("decodedblocku8 = {:?}", decodedblocku8);
                    
                // // 写入数据库
                // // 缓冲
                // let mut batch = DBTransaction::new();
                // let mut write_datablock = self.datablockmap.write();
                // // batch.extend_with_cache(
                // batch.write_with_cache(
                //     mydb::MYCOL_DATABLOCKS, 
                //     // cache,
                //     &mut *write_datablock, 
                //     format!("{0}_{1}", sectionid, datablockid).clone(), 
                //     decodedblocku8.clone(),
                //     // datablockmap,
                //     CacheUpdatePolicy::Overwrite,
                // );  
                // self.mydb.write(batch).expect("DB write failed.");

                // 转换成string格式
                let strdatablock = String::from_utf8(decodedblocku8).unwrap();
                info!("strdatablock={}", strdatablock);

                //转换成数据块DataBlock格式
                let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

                // 从数据块中取出序列化的区块
                let blocks = desedatablock.databody;

                let blockid: usize;
                if section_size % k == 0 {
                    blockid = ((number % section_size) % (section_size / k)) as usize;
                } else {
                    if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
                        blockid = ((number % section_size) % (section_size / k + 1)) as usize;
                    } else{
                        blockid = ((number % section_size - (section_size / k + 1) * (section_size % k)) % (section_size / k)) as usize;
                    }
                }

                let serblock = blocks.get(blockid).unwrap();
                block = Some(serblock.blockbody.clone());
            }
        }
        block
    }
    
    
    /// Get block by height
    pub fn block_by_height(&self, number: BlockNumber) -> Option<Block> {
        
        // let m = self.m;
        // let w = self.w;
        if number > self.get_old_height() { //尚未纠删码编码
            return None;
        }
        let section_size = self.section_size;
        let mut block;
        // 计算对应的数据块id
        // let number = number - 1; // height = number - 1
        info!("block_by_height: height={}", number);
        let sectionid = number / section_size; //能不能被整除一样
        let k = self.get_kmap(sectionid).unwrap().clone();
        info!("k = {}", k);
        let datablockid:u64; // 区间中的第几块
        if section_size % k == 0 {
            datablockid = (number % section_size) / (section_size / k);
        } else {
            if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
                datablockid = (number % section_size) / (section_size / k + 1);
            } else{
                datablockid = section_size % k + (number % section_size - (section_size / k + 1) * (section_size % k)) / (section_size / k);
            }
        }

        // if (number % section_size) % (section_size / k) == 0 {
        // } else {
        //     datablockid += 1;
        // }
        info!("sectionid = {}, datablockid = {}", sectionid, datablockid);
        let strdatablockid = format!("{0}_{1}", sectionid, datablockid);
        // 获取区块所在节点的序号
        let addresses = self.get_blocknodesmap(sectionid).unwrap().clone();
        info!("all address:{:?}", addresses);
        let address = addresses.nodes.get(datablockid as usize).unwrap().clone(); // 一个区间中的第n块的地址
        if address == self.address { //如果请求的块存在本地
            // 根据数据块id获取数据块
            match self.get_datablockmap(strdatablockid.clone()) {
                Some(datablock) => { // 从本地获取到数据块
                    // 读取一个数据块中的有效信息
                    let bufdatablock = datablock.clone();
                    // info!("bufdatablock = {:?}", bufdatablock);
                    
                    // 转换成string格式
                    let strdatablock = String::from_utf8(bufdatablock).unwrap();
                    info!("strdatablock={}", strdatablock);

                    //转换成数据块DataBlock格式
                    let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

                    // 从数据块中取出序列化的区块
                    let blocks = desedatablock.databody;

                    let blockid:usize;

                    if section_size % k == 0 {
                        blockid = (number % (section_size / k)) as usize;
                    } else {
                        if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
                            blockid = ((number % section_size) % (section_size / k + 1)) as usize;
                        } else{
                            blockid = ((number % section_size - (section_size / k + 1) * (section_size % k)) % (section_size / k)) as usize;
                        }
                    }

                    info!("blockid = {}", blockid);
                    let serblock = blocks.get(blockid).unwrap();
                    // info!("serblock get ok! blockid = {}, height = {}", blockid, serblock.id);
                    // info!("serblock.blockbody = {:?}", serblock.blockbody);
                    let bufblock = &serblock.blockbody;
                    let slice = bufblock.as_slice();
                    let untrusted_rlp = UntrustedRlp::new(slice);
                    block = Block::decode(&untrusted_rlp).unwrap();
                    // info!("RLP  New Block={:?}", block);
                }
                None => { //未从本地获取到区块->区块丢失->纠删码解码
                    let mut blocku8:Vec<u8> = Vec::new();  //u8类型 一维数据块 传入参数
                    let mut i:usize = 0;
                    let maxdatablocksize = self.get_maxblocknodesize(sectionid).unwrap();
                    let k = self.get_kmap(sectionid).unwrap();
                    let m = self.get_mmap(sectionid).unwrap();
                    let mut erasures:Vec<i32> = Vec::with_capacity((k + m) as usize); //记录缺失的数据块
                    let mut lostdatablock:Vec<u8> = Vec::with_capacity(maxdatablocksize);
                    for _i in 0..maxdatablocksize {
                        lostdatablock.push(0x00);
                    }
                    for addr in addresses.nodes {
                        if addr == address { //本节点丢失的区块
                            blocku8.append(&mut lostdatablock.clone());  
                            erasures.push(i as i32);
                            info!("1 blocku8.size = {:?}", blocku8.len());
                        } else {
                            match self.de_reqwest_block(addr, format!("{0}_{1}", sectionid, i)) {
                                Some(datablock) => {
                                    info!("datablock.size = {:?}", datablock.len());
                                    let str_datablock = String::from_utf8(datablock.clone()).unwrap();
                                    info!("str_datablock={}", str_datablock);
    
                                    blocku8.append(&mut datablock.clone());
                                    let mut j = datablock.len();
                                    while j < maxdatablocksize {
                                        blocku8.push(0x00);
                                        j += 1;
                                    }
                                    info!("2 blocku8.size = {:?}", blocku8.len());
                                }
                                None => {
                                    blocku8.append(&mut lostdatablock);
                                    erasures.push(i as i32);
                                    info!("3 blocku8.size = {:?}", blocku8.len());
                                }
                            }
                        }
                        i += 1;
                    }
                    
                    erasures.push(-1);
                    
                    info!("erasures = {:?}", erasures);
                    info!("maxdatablocksize = {:?}", maxdatablocksize);
                    info!("blocku8.size = {:?}", blocku8.len());
                    let mut coding = blocku8.split_off(maxdatablocksize * k as usize);
                    let praw_coding = coding.as_mut_ptr();
                    let strdatablock = String::from_utf8(blocku8.clone()).unwrap();
                    info!("strdatablock={:?}", strdatablock);
                    // info!("blocku8 = {:?}", blocku8);
                    let puncode_stream = blocku8.as_mut_ptr();
                    let raw_datablocksize = maxdatablocksize;
                    let perasures = erasures.as_mut_ptr();

                    unsafe {
                        for _i in 0..erasures.len() {
                            info!("{:?}", *perasures.add(_i));
                        }
                    }

                    decoder_clothes_w(k, m, self.w, puncode_stream, praw_coding , raw_datablocksize, perasures);

                    info!("decoder ok!");

                    let mut decodedblocku8 = blocku8.clone();

                    // ？？？？？？？？？
                    let strdatablock = String::from_utf8(decodedblocku8.clone()).unwrap();
                    info!("strdatablock={:?}", strdatablock);

                    let strcoding = String::from_utf8(coding.clone()).unwrap();
                    info!("strcoding={:?}", strcoding);

                    for _i in 0..datablockid {
                        decodedblocku8 = decodedblocku8.split_off(maxdatablocksize);
                    }

                    // 读取一个数据块中的有效信息
                    let datablocksize = self.get_datablocksize(format!("{0}_{1}", sectionid, datablockid)).unwrap().clone(); 
                    info!("datablocksize = {}", datablocksize);
                    decodedblocku8.split_off(datablocksize);
                    info!("decodedblocku8.size = {}", decodedblocku8.len());
                
                    // info!("decodedblocku8 = {:?}", decodedblocku8);
                    
                    // // 写入数据库
                    // // 缓冲
                    // let mut batch = DBTransaction::new();
                    // let mut write_datablock = self.datablockmap.write();
                    // // batch.extend_with_cache(
                    // batch.write_with_cache(
                    //     mydb::MYCOL_DATABLOCKS, 
                    //     // cache,
                    //     &mut *write_datablock, 
                    //     format!("{0}_{1}", sectionid, datablockid).clone(), 
                    //     decodedblocku8.clone(),
                    //     // datablockmap,
                    //     CacheUpdatePolicy::Overwrite,
                    // );  
                    // self.mydb.write(batch).expect("DB write failed.");

                    // 转换成string格式
                    let strdatablock = String::from_utf8(decodedblocku8).unwrap();
                    info!("strdatablock={}", strdatablock);

                    //转换成数据块DataBlock格式
                    let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

                    // 从数据块中取出序列化的区块
                    let blocks = desedatablock.databody;

                    let blockid: usize;
                    if section_size % k == 0 {
                        blockid = ((number % section_size) % (section_size / k)) as usize;
                    } else {
                        if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
                            blockid = ((number % section_size) % (section_size / k + 1)) as usize;
                        } else{
                            blockid = ((number % section_size - (section_size / k + 1) * (section_size % k)) % (section_size / k)) as usize;
                        }
                    }

                    let serblock = blocks.get(blockid).unwrap();
                    let bufblock = &serblock.blockbody;
                    let slice = bufblock.as_slice();
                    let untrusted_rlp = UntrustedRlp::new(slice);
                    block = Block::decode(&untrusted_rlp).unwrap();
                }
            }
        } else {
            info!("in other node");
            block = match self.reqwest_block(address, number) {
                Some(block) => {
                    block
                }
                None => { // 解码
                    let mut blocku8:Vec<u8> = Vec::new();  //u8类型 一维数据块 传入参数
                    let mut i:usize = 0;
                    let maxdatablocksize = self.get_maxblocknodesize(sectionid).unwrap();
                    let k = self.get_kmap(sectionid).unwrap();
                    let m = self.get_mmap(sectionid).unwrap();
                    let mut erasures:Vec<i32> = Vec::with_capacity((k + m) as usize); //记录缺失的数据块
                    let mut lostdatablock:Vec<u8> = Vec::with_capacity(maxdatablocksize);
                    for _i in 0..maxdatablocksize {
                        lostdatablock.push(0x00);
                    }
                    for addr in addresses.nodes {
                        if addr == address {
                            blocku8.append(&mut lostdatablock.clone());  
                            erasures.push(i as i32);
                            info!("1 blocku8.size = {:?}", blocku8.len());
                        } else if addr == self.address {
                            match self.get_datablockmap(format!("{0}_{1}", sectionid, i)) {
                                Some(datablock) => {
                                    blocku8.append(&mut datablock.clone());
                                    let mut j = datablock.len();
                                    while j < maxdatablocksize {
                                        blocku8.push(0x00);
                                        j += 1;
                                    }
                                }
                                None => {
                                    blocku8.append(&mut lostdatablock.clone());
                                    erasures.push(i as i32);
                                }
                            }
                            info!("2 blocku8.size = {:?}", blocku8.len());
                        } else {
                            match self.de_reqwest_block(addr, format!("{0}_{1}", sectionid, i)) {
                                Some(datablock) => {
                                    blocku8.append(&mut datablock.clone());
                                    let mut j = datablock.len();
                                    while j < maxdatablocksize {
                                        blocku8.push(0x00);
                                        j += 1;
                                    }
                                    info!("3 blocku8.size = {:?}", blocku8.len());
                                }
                                None => {
                                    blocku8.append(&mut lostdatablock.clone());
                                    erasures.push(i as i32);
                                    info!("4 blocku8.size = {:?}", blocku8.len());
                                }
                            }
                        }
                        i += 1;
                    }
                    
                    erasures.push(-1);

                    info!("erasures = {:?}", erasures);
                    info!("maxdatablocksize = {:?}", maxdatablocksize);
                    info!("blocku8.size = {:?}", blocku8.len());
                    let mut coding = blocku8.split_off(maxdatablocksize * k as usize);
                    let praw_coding = coding.as_mut_ptr();
                    let strdatablock = String::from_utf8(blocku8.clone()).unwrap();
                    info!("strdatablock={:?}", strdatablock);
                    // info!("blocku8 = {:?}", blocku8);
                    let puncode_stream = blocku8.as_mut_ptr();
                    let raw_datablocksize = maxdatablocksize;
                    let perasures = erasures.as_mut_ptr();

                    unsafe {
                        for _i in 0..erasures.len() {
                            info!("{:?}", *perasures.add(_i));
                        }
                    }

                    decoder_clothes_w(k, m, self.w, puncode_stream, praw_coding , raw_datablocksize, perasures);

                    info!("decoder ok!");
                    let mut decodedblocku8 = blocku8.clone();

                    // ？？？？？？？？？
                    let strdatablock = String::from_utf8(decodedblocku8.clone()).unwrap();
                    info!("strdatablock={:?}", strdatablock);

                    let strcoding = String::from_utf8(coding.clone()).unwrap();
                    info!("strcoding={:?}", strcoding);

                    for _i in 0..datablockid {
                        decodedblocku8 = decodedblocku8.split_off(maxdatablocksize);
                    }

                    // 读取一个数据块中的有效信息
                    let datablocksize = self.get_datablocksize(format!("{0}_{1}", sectionid, datablockid)).unwrap().clone(); 
                    info!("datablocksize = {}", datablocksize);
                    decodedblocku8.split_off(datablocksize);
                    info!("decodedblocku8.size = {}", decodedblocku8.len());
                
                    // info!("decodedblocku8 = {:?}", decodedblocku8);
                    
                    // // 写入数据库
                    // // 缓冲
                    // let mut batch = DBTransaction::new();
                    // let mut write_datablock = self.datablockmap.write();
                    // // batch.extend_with_cache(
                    // batch.write_with_cache(
                    //     mydb::MYCOL_DATABLOCKS, 
                    //     // cache,
                    //     &mut *write_datablock, 
                    //     format!("{0}_{1}", sectionid, datablockid).clone(), 
                    //     decodedblocku8.clone(),
                    //     // datablockmap,
                    //     CacheUpdatePolicy::Overwrite,
                    // );  
                    // self.mydb.write(batch).expect("DB write failed.");

                    // 转换成string格式
                    let strdatablock = String::from_utf8(decodedblocku8).unwrap();
                    info!("strdatablock={}", strdatablock);

                    //转换成数据块DataBlock格式
                    let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

                    // 从数据块中取出序列化的区块
                    let blocks = desedatablock.databody;

                    let blockid: usize;
                    if section_size % k == 0 {
                        blockid = ((number % section_size) % (section_size / k)) as usize;
                    } else {
                        if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
                            blockid = ((number % section_size) % (section_size / k + 1)) as usize;
                        } else{
                            blockid = ((number % section_size - (section_size / k + 1) * (section_size % k)) % (section_size / k)) as usize;
                        }
                    }

                    let serblock = blocks.get(blockid).unwrap();
                    let bufblock = &serblock.blockbody;
                    let slice = bufblock.as_slice();
                    let untrusted_rlp = UntrustedRlp::new(slice);
                    block = Block::decode(&untrusted_rlp).unwrap();
                    block
                }
            }
        }

        // // 根据数据块id获取数据块
        // match self.get_datablockmap(strdatablockid.clone()) {
        //     Some(datablock) => {
        //         // 读取一个数据块中的有效信息
        //         let bufdatablock = datablock.clone();
        //         // info!("bufdatablock = {:?}", bufdatablock);
                
        //         // 转换成string格式
        //         let strdatablock = String::from_utf8(bufdatablock).unwrap();
        //         info!("strdatablock={}", strdatablock);

        //         //转换成数据块DataBlock格式
        //         let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

        //         // 从数据块中取出序列化的区块
        //         let blocks = desedatablock.databody;

        //         let blockid:usize;

        //         if section_size % k == 0 {
        //             blockid = (number % (section_size / k)) as usize;
        //         } else {
        //             if (number % section_size) < (section_size / k + 1) * (section_size % k) {              
        //                 blockid = ((number % section_size) % (section_size / k + 1)) as usize;
        //             } else{
        //                 blockid = ((number % section_size - (section_size / k + 1) * (section_size % k)) % (section_size / k)) as usize;
        //             }
        //         }

        //         info!("blockid = {}", blockid);
        //         let serblock = blocks.get(blockid).unwrap();
        //         // info!("serblock get ok! blockid = {}, height = {}", blockid, serblock.id);
        //         // info!("serblock.blockbody = {:?}", serblock.blockbody);
        //         let bufblock = &serblock.blockbody;
        //         let slice = bufblock.as_slice();
        //         let untrusted_rlp = UntrustedRlp::new(slice);
        //         block = Block::decode(&untrusted_rlp).unwrap();
        //         // info!("RLP  New Block={:?}", block);
        //     }
        //     None =>{
        //         // // 获取其他节点的Address和IP:Port
        //         // let upstream = UpStream::new(String::from(format!("{}:{}", "127.0.0.1", self.netconf.port)) , Duration::new(100, 0));
        //         // let req = rpc_request::PeersInfoParams::new().into_request(1);
        //         // info!("req = {:?}", req);
        //         // define_reply_type!(ReplyType, rpc_types::Data);
        //         // let rpc_cli = RpcClient::create(upstream);
        //         // let body: String = req.into();
        //         // let data = rpc_cli.do_post(&body);
        //         // info!("data = {:?}", data);
        //         // match data {
        //         //     Ok(chunk) => {
        //         //         let strreply = format!("{:?}", chunk);
        //         //         let slicereply = strreply.as_bytes();
        //         //         info!("slicereply = {:?}", slicereply);
        //         //         let reply: ReplyType = serde_json::from_slice(&data);
        //         //         match reply {
        //         //             Ok() => {
        //         //             }
        //         //             Err(err) => {
        //         //                 Err(Error::Parse);
        //         //             }
        //         //         }
        //         //     }
        //         //     Err(e) => {
        //         //         info!("err = {:?}", e)
        //         //     }
        //         // }
                
        //         // 获取区块所在节点的序号
        //         let addresses = self.get_blocknodesmap(sectionid).unwrap().clone();
        //         info!("all address:{:?}", addresses);
        //         let address = addresses.nodes.get(datablockid as usize).unwrap().clone(); // 一个区间中的第n块的地址
        //         // let allnodes = self.get_allnodes().nodes;
        //         // let len = allnodes.len();
        //         // info!("len = {}", len);
        //         // let mut index:usize = 0;
        //         // for i in 0..len {
        //         //     index = i;
        //         //     info!("&address = {:?}", &address);
        //         //     info!("allnodes.get(i) = {:?}", allnodes.get(i));
        //         //     if Some(&address) == allnodes.get(i) {
        //         //         break;
        //         //     }
        //         // }

        //         // 发送请求
        //         block = match self.reqwest_block(address, number) {
        //             Some(_block) => {
        //                 let mut blocku8:Vec<u8> = Vec::new();  //u8类型 一维数据块 传入参数
        //                 let mut i:usize = 0;
        //                 let maxdatablocksize = self.get_maxblocknodesize(sectionid).unwrap();
        //                 let k = self.get_kmap(sectionid).unwrap();
        //                 let m = self.get_mmap(sectionid).unwrap();
        //                 let mut erasures:Vec<i32> = Vec::with_capacity((k + m) as usize); //记录缺失的数据块
        //                 let perasures = erasures.as_mut_ptr();
        //                 let mut erasuresnum = 0;
        //                 let mut lostdatablock:Vec<u8> = Vec::with_capacity(maxdatablocksize);
        //                 for _i in 0..maxdatablocksize {
        //                     lostdatablock.push(0x00);
        //                 }
        //                 for addr in addresses.nodes {
        //                     if addr == address {
        //                         blocku8.append(&mut lostdatablock);   
        //                         unsafe {
        //                             *perasures.add(erasuresnum) = i as i32;
        //                             erasuresnum += 1;
        //                         }
        //                         i += 1;
        //                         info!("1 blocku8.size = {:?}", blocku8.len());
        //                         continue;
        //                     }
        //                     if addr == self.address {
        //                         match self.get_datablockmap(format!("{0}_{1}", sectionid, i)) {
        //                             Some(datablock) => {
        //                                 blocku8.append(&mut datablock.clone());
        //                                 let mut j = datablock.len();
        //                                 while j < maxdatablocksize {
        //                                     blocku8.push(0x00);
        //                                     j += 1;
        //                                 }
        //                             }
        //                             None => {
        //                                 unsafe { 
        //                                     blocku8.append(&mut Vec::with_capacity(maxdatablocksize));
        //                                     *perasures.add(erasuresnum) = i as i32;
        //                                     erasuresnum += 1;
        //                                 }
        //                             }
        //                         }
        //                         i += 1;
        //                         info!("2 blocku8.size = {:?}", blocku8.len());
        //                         continue;
        //                     }
        //                     match self.de_reqwest_block(addr, format!("{0}_{1}", sectionid, i)) {
        //                         Some(datablock) => {
        //                             blocku8.append(&mut datablock.clone());
        //                             let mut j = datablock.len();
        //                             while j < maxdatablocksize {
        //                                 blocku8.push(0x00);
        //                                 j += 1;
        //                             }
        //                             info!("3 blocku8.size = {:?}", blocku8.len());
        //                         }
        //                         None => {
        //                             blocku8.append(&mut Vec::with_capacity(maxdatablocksize));
        //                             unsafe { 
        //                                 *perasures.add(erasuresnum) = i as i32;
        //                                 erasuresnum += 1;
        //                             }
        //                             info!("4 blocku8.size = {:?}", blocku8.len());
        //                         }
        //                     }
        //                     i += 1;
        //                 }
                        
        //                 unsafe {
        //                     *perasures.add(erasuresnum) = -1;
        //                 }

        //                 info!("maxdatablocksize = {:?}", maxdatablocksize);
        //                 info!("blocku8.size = {:?}", blocku8.len());
        //                 let praw_coding = blocku8.split_off(maxdatablocksize * k as usize).as_mut_ptr();
        //                 info!("blocku8.size = {:?}", blocku8.len());
        //                 let puncode_stream = blocku8.as_mut_ptr();
        //                 let raw_datablocksize = maxdatablocksize;

        //                 info!("perasures = {:?}", perasures);

        //                 decoder_clothes_w(k, m, self.w, puncode_stream, praw_coding , raw_datablocksize, perasures);

        //                 info!("decoder ok!");
        //                 let mut decodedblocku8 = blocku8.clone();
        //                 for _i in 0..datablockid {
        //                     let _vec = decodedblocku8.split_off(maxdatablocksize);
        //                 }

        //                 // 读取一个数据块中的有效信息
        //                 let datablocksize = self.get_datablocksize(format!("{0}_{1}", sectionid, datablockid)).unwrap().clone();
        //                 let bufdatablock = decodedblocku8.split_off(datablocksize);
                        
        //                 info!("bufdatablock = {:?}", bufdatablock);
                        
        //                 // 转换成string格式
        //                 let strdatablock = String::from_utf8(bufdatablock).unwrap();
        //                 info!("strdatablock={}", strdatablock);

        //                 //转换成数据块DataBlock格式
        //                 let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

        //                 // 从数据块中取出序列化的区块
        //                 let blocks = desedatablock.databody;

        //                 let blockid = (number % (section_size / k) - 1 ) as usize;
        //                 let serblock = blocks.get(blockid).unwrap();
        //                 let bufblock = &serblock.blockbody;
        //                 let slice = bufblock.as_slice();
        //                 let untrusted_rlp = UntrustedRlp::new(slice);
        //                 block = Block::decode(&untrusted_rlp).unwrap();
        //                 block
        //             }
        //             None => { // 解码
        //                 block = Block::default();
        //                 block
        //             }
        //         }

                // let peerconf = self.get_addr_peer().get(&address).unwrap().clone();
                // info!("peerconf{:?}", peerconf.clone());
                // // 构建通信的通道

                // // let base_url = "http://jsonplaceholder.typicode.com/";
                // // let url = format!("{}posts", base_url);
                // // let upstream = UpStream::new(url, Duration::new(100, 0));
                // let upstream = UpStream::new(String::from(format!("http://{}:{}", peerconf.clone().ip.unwrap(), peerconf.clone().port.unwrap())) , Duration::new(100, 0));
                // // let upstream = UpStream::new(String::from(format!("http://{}:{}", "127.0.0.1", "1337")) , Duration::new(100, 0));
                // let upstream = &upstream;
                // let memnumber = number.to_le_bytes().to_vec();
                // let mut vecnumber: Vec<u8> = Vec::with_capacity(32);
                // for i in 0..32 {
                //     if i < memnumber.len() {
                //         vecnumber.push(memnumber.get(i).unwrap().clone());
                //     } else {
                //         vecnumber.push(0);
                //     }
                // }
                // vecnumber.reverse();
                // let rpcnumber  = rpc_types::Quantity::new(U256::from(vecnumber.as_slice()));
                // let req = rpc_request::GetBlockByNumberParams::new(RpcBlockNumber::new(rpcnumber), rpc_types::Boolean::new(true)).into_request(1);
                // info!("req = {:?}", req);
                
                // let rpc_cli = RpcClient::create(upstream);
                // let body: String = req.into();
                // // let data = rpc_cli.do_post(&body)?;
                // // let reply: ReplyType = serde_json::from_slice(&data).map_err(|_| {
                // //     error!(
                // //         "send {:?} return error: {:?}",
                // //         &body,
                // //         ::std::str::from_utf8(&data)
                // //     );
                // //     Error::Parse
                // // })?;
                // // trace!("get reply {:?}.", reply);

                // let data = rpc_cli.do_post(&body);
                // // info!("data = {:?}", data);
                // match data {
                //     Ok(chunk) => {
                //         let slicereply = chunk.as_ref();
                //         info!("slicereply = {:?}", slicereply);
                        
                //         let replyvec = slicereply.to_vec();
                //         let strreply = String::from_utf8(replyvec).unwrap();
                //         let strreply = strreply.as_str();
                //         info!("strreply = {:?}", strreply);

                //         let reply = match serde_json::from_slice::<ReplyType>(slicereply) {
                //             Ok(reply) => {
                //                 Ok(reply)
                //             }
                //             Err(e) => {
                //                 info!("parse error :{:?}", e);
                //                 Err(Error::Parse)
                //             }
                //         };
                        
                //         // info!("reply = {:?}", reply.result);
                //         info!("reply = {:?}", reply);
                //     }
                //     Err(e) => {
                //         info!("err = {:?}", e)
                //     }
                // }
                

                // let reply: ReplyType = serde_json::from_slice(&data).map_err(|_| {
                //     error!(
                //         "send {:?} return error: {:?}",
                //         &body,
                //         ::std::str::from_utf8(&data)
                //     );
                //     Error::Parse
                // });
                // trace!("get reply {:?}.", reply);
                // let result = reply.result;

                // let result = rpc_send_and_get_result_from_reply!(upstream, req, rpc_types::Data);
                // info!("result={:?}", result);

                // 从其他节点获取区块

                // // 解码
                // let mut erasures:Vec<i32> = Vec::with_capacity((k + m) as usize);
                // let perasures = erasures.as_mut_ptr();
                // let mut erasuresnum = 0;
                // // unsafe {
                // //     *perasures = 0;
                // //     *perasures.add(1) = 1;
                // //     *perasures.add(2) = -1;
                // // }
                
                // let maxdatablocksize = self.get_datablocksizemap().get(&format!("{0}_{1}", sectionid, k + m - 1)).unwrap().clone();
                // let mut blocku8:Vec<u8> = Vec::new();  //u8类型 一维数据块 传入参数
                // for i in 0..k + m {
                //     match self.get_datablockmap().get(&format!("{0}_{1}", sectionid, i)) {
                //         Some(datablock) => {
                //             blocku8.append(&mut datablock.clone());
                //             let mut j = self.get_datablocksizemap().get(&format!("{0}_{1}", sectionid, i)).unwrap().clone();
                //             while j < maxdatablocksize {
                //                 blocku8.push(0x00);
                //                 j += 1;
                //             }
                //         } 
                //         None => {
                //             blocku8.append(&mut Vec::with_capacity(maxdatablocksize));
                //             unsafe { 
                //                 *perasures.add(erasuresnum) = i as i32;
                //                 erasuresnum += 1;
                //             }
                //         }
                //     }
                // }

                // unsafe {
                //     *perasures.add(erasuresnum) = -1;
                // }

                // let puncode_stream = blocku8.split_off(maxdatablocksize * k as usize).as_mut_ptr();
                // let praw_coding = blocku8.as_mut_ptr();
                // let raw_datablocksize = maxdatablocksize;

                // decoder_clothes_w(k, m, w, puncode_stream, praw_coding , raw_datablocksize, perasures);

                // let mut decodedblocku8 = blocku8.clone();
                // for _i in 0..datablockid {
                //     let _vec = decodedblocku8.split_off(maxdatablocksize);
                // }

                // // 读取一个数据块中的有效信息
                // let datablocksize = self.get_datablocksizemap().get(&format!("{0}_{1}", sectionid,datablockid)).unwrap().clone();
                // let bufdatablock = decodedblocku8.split_off(datablocksize);
                
                // info!("bufdatablock = {:?}", bufdatablock);
                
                // // 转换成string格式
                // let strdatablock = String::from_utf8(bufdatablock).unwrap();
                // info!("strdatablock={}", strdatablock);

                // //转换成数据块DataBlock格式
                // let desedatablock:DataBlock = serde_json::from_str(&strdatablock).unwrap();

                // // 从数据块中取出序列化的区块
                // let blocks = desedatablock.databody;

                // let blockid = (number % (section_size / k) - 1 ) as usize;
                // let serblock = blocks.get(blockid).unwrap();
                // let bufblock = &serblock.blockbody;
                // let slice = bufblock.as_slice();
                // let untrusted_rlp = UntrustedRlp::new(slice);
                // block = Block::decode(&untrusted_rlp).unwrap();

                // block = Block::default();
            // }
        // }
        Some(block)
    }

    // 发送请求
    pub fn reqwest_block(&self,
                address: H160, 
                number:u64) -> Option<Block> {

        let u8number = number.to_string().as_bytes().to_vec();
        // u32 to vec<u8>
        // let bytes = index.to_be_bytes().to_vec();


        // [u8] to u32
        // let ptr :*const u8 = pack_data.as_ptr();
        // let ptr :*const u32 = ptr as *const u32;
        // let s = unsafe{ *ptr};
        // println!("{:?}", s);


        // let header = &*self.current_header.read();
        // let version_opt = self.version.read();

        // if self.nodes.read().is_empty() || version_opt.is_none() {
        //     trace!("delivery_current_rich_status : node list or version is not ready!");
        //     return;
        // }
        // let current_hash = header.hash().unwrap();
        // let current_height = header.number();
        // let nodes: Vec<Address> = self.nodes.read().clone();
        // let validators: Vec<Address> = self.validators.read().clone();
        // let block_interval = self.block_interval.read();

        // let mut rich_status = ProtoRichStatus::new();
        // rich_status.set_hash(current_hash.0.to_vec());
        // rich_status.set_height(current_height);
        // rich_status.set_nodes(nodes.into_iter().map(|address| address.to_vec()).collect());
        // rich_status.set_validators(validators.into_iter().map(|a| a.to_vec()).collect());
        // rich_status.set_interval(*block_interval);
        // rich_status.set_version(version_opt.unwrap());

        info!("address = {:?}; u8number = {:?}", address, u8number);
        let netmsg = EcNetMessage::new(address, u8number.clone());
        let u8netmsg = serde_json::to_string(&netmsg).unwrap().as_bytes().to_vec();
        let msg = Message::init(OperateType::Broadcast, 0, u8netmsg.into());
        self.ctx_pub
            .send((
                routing_key!(Chain >> RawBytes).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
        let mut block:Option<Block>;
        let sub_ec = self.crx_sub_ec.clone();
        // thread::spawn(move || loop {
            let (key, body) = sub_ec.recv().unwrap();
            let mut msg = Message::try_from(&body).unwrap();
            // send_message(&nodes_mgr_client, key, msg);
            info!("Key in encoder:{:?}", key);
            info!("Message in encoder:{:?}", msg);
            let raw = msg.take_raw_bytes().unwrap();
            
            if raw == u8number {
                block = None;
            } else {
                let slice = raw.as_slice();
                let untrusted_rlp = UntrustedRlp::new(slice);
                block = Some(Block::decode(&untrusted_rlp).unwrap());
            }
        // });
        block
    }

    // 解码 发送请求
    pub fn de_reqwest_block(&self,
            address: H160, 
            strdatablockid: String) -> Option<Vec<u8>> {
        let u8datablockid = strdatablockid.as_bytes().to_vec();

        info!("address = {:?}; u8datablockid = {:?}", address, u8datablockid);
        let netmsg = EcNetMessage::new(address, u8datablockid.clone());
        let u8netmsg = serde_json::to_string(&netmsg).unwrap().as_bytes().to_vec();
        let msg = Message::init(OperateType::Broadcast, 0, u8netmsg.into());
        self.ctx_pub
            .send((
                routing_key!(Chain >> RawBytes).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
        let mut block:Option<Vec<u8>>;
        let sub_ec = self.crx_sub_ec.clone();
        // thread::spawn(|| {
            let (key, body) = sub_ec.recv().unwrap();
            let mut msg = Message::try_from(&body).unwrap();
            // send_message(&nodes_mgr_client, key, msg);
            info!("Key in encoder:{:?}", key);
            // info!("Message in encoder:{:?}", msg);
            let raw = msg.take_raw_bytes().unwrap();
            
            if raw == u8datablockid {
                block = None;
            } else {
                let slice = raw.as_slice();
                block = Some(slice.to_vec());
            }
        // });

        block
    }


    // fn reply_syn_req(&self, sync_req: SyncRequest, origin: u32) {
    //     let mut sync_req = sync_req;
    //     let heights = sync_req.take_heights();
    //     debug!(
    //         "sync: receive sync from node {:?}, height lists = {:?}",
    //         origin, heights
    //     );

    //     let res_vec = self.sync_response(heights);

    //     debug!(
    //         "sync: reply node = {}, response blocks len = {}",
    //         origin,
    //         res_vec.get_blocks().len()
    //     );
    //     if !res_vec.get_blocks().is_empty() {
    //         let msg = Message::init(OperateType::Single, origin, res_vec.into());
    //         trace!(
    //             "sync: origin {:?}, chain.blk: OperateType {:?}",
    //             origin,
    //             OperateType::Single
    //         );
    //         self.ctx_pub
    //             .send((
    //                 routing_key!(Chain >> SyncResponse).into(),
    //                 msg.try_into().unwrap(),
    //             ))
    //             .unwrap();
    //     }
    // }


    /// Get block header by BlockId
    pub fn block_header(&self, id: BlockId) -> Option<Header> {
        match id {
            BlockId::Hash(hash) => self.block_header_by_hash(hash),
            BlockId::Number(number) => self.block_header_by_height(number),
            BlockId::Earliest => self.block_header_by_height(0),
            BlockId::Latest => self.block_header_by_height(self.get_latest_height()),
            BlockId::Pending => self.block_header_by_height(self.get_pending_height()),
        }
    }

    /// Get block header by hash
    pub fn block_header_by_hash(&self, hash: H256) -> Option<Header> {
        let result;
        match self.block_by_hash(hash) {
            Some(h) => {
                result = Some(h.header().clone());
            }
            None => {
                result = None;
            }
        }
        result
    }

    /// Get full transaction by hash
    pub fn full_transaction(&self, hash: TransactionId) -> Option<FullTransaction> {
        self.transaction_address(hash).and_then(|addr| {
            let index = addr.index;
            let hash = addr.block_hash;
            self.block_by_hash(hash).map(|block| {
                let transactions = block.body().transactions();
                let tx = transactions[index].protobuf();
                let mut full_ts = FullTransaction::new();
                full_ts.set_transaction(tx);
                full_ts.set_block_number(block.number());
                full_ts.set_block_hash(hash.to_vec());
                full_ts.set_index(index as u32);
                full_ts
            })
        })
    }

    /// Get address of transaction by hash.
    fn transaction_address(&self, hash: TransactionId) -> Option<TransactionAddress> {
        let result;
        match self.get_transaction_addresses().get(&hash) {
            Some(h) => {
                result = Some(h.clone());
            }
            None => {
                result = None;
            }
        }
        result
    }

    /// 获取交易提交者
    pub fn get_transaction_proof(&self, hash: TransactionId) -> Option<(Vec<u8>)> {
        self.transaction_address(hash)
            .and_then(|addr| {
                self.block_by_hash(addr.block_hash)
                    .map(|block| (addr, block))
            })
            .and_then(|(addr, block)| {
                self.block_receipts(addr.block_hash)
                    .map(|receipts| (addr.index, block, receipts))
            })
            .and_then(|(index, block, receipts)| {
                receipts
                    .receipts
                    .get(index)
                    .and_then(|receipt| {
                        if receipt.transaction_hash == hash {
                            Some(receipt)
                        } else {
                            None
                        }
                    })
                    .and_then(|receipt| {
                        cita_merklehash::Tree::from_hashes(
                            receipts
                                .receipts
                                .iter()
                                .map(|r| r.rlp_bytes().into_vec().crypt_hash())
                                .collect::<Vec<_>>(),
                            cita_merklehash::merge,
                        )
                        .get_proof_by_input_index(index)
                        .map(|receipt_proof| (index, block, receipt.clone(), receipt_proof))
                    })
            })
            .and_then(|(index, block, receipt, receipt_proof)| {
                block.body().transactions().get(index).map(|tx| {
                    (
                        tx.clone(),
                        receipt,
                        receipt_proof.into(),
                        block.header().clone(),
                    )
                })
            })
            .and_then(|(tx, receipt, receipt_proof, block_header)| {
                self.block_by_height(block_header.number() + 1)
                    .map(|next_block| {
                        (
                            tx,
                            receipt,
                            receipt_proof,
                            block_header,
                            next_block.header().proposal(),
                        )
                    })
            })
            .and_then(
                |(tx, receipt, receipt_proof, block_header, next_proposal_header)| {
                    self.block_by_height(next_proposal_header.number() + 1)
                        .map(|third_block| {
                            (
                                tx,
                                receipt,
                                receipt_proof,
                                block_header,
                                next_proposal_header,
                                third_block.header().proof().clone(),
                            )
                        })
                },
            )
            .map(
                |(
                    tx,
                    receipt,
                    receipt_proof,
                    block_header,
                    next_proposal_header,
                    proposal_proof,
                )| {
                    core::libchain::chain::TxProof {
                        tx,
                        receipt,
                        receipt_proof,
                        block_header,
                        next_proposal_header,
                        proposal_proof,
                    }
                    .rlp_bytes()
                    .into_vec()
                },
            )
    }

    /// Get receipts of block with given hash.
    pub fn block_receipts(&self, hash: H256) -> Option<BlockReceipts> {
        let result;
        match self.get_block_receipts().get(&hash) {
            Some(h) => {
                result = Some(h.clone());
            }
            None => {
                result = None;
            }
        }
        result
    }


    pub fn localized_receipt(&self, id: TransactionId) -> Option<LocalizedReceipt> {
        trace!("Get receipt id: {:?}", id);

        let address = match self.transaction_address(id) {
            Some(addr) => addr,
            _ => return None,
        };
        let hash = address.block_hash;
        let index = address.index;

        let mut receipts = match self.block_receipts(hash) {
            Some(r) => r.receipts,
            _ => return None,
        };

        receipts.truncate(index + 1);
        let last_receipt = receipts.pop().expect("Current receipt is provided; qed");

        let prior_quota_used = match receipts.last() {
            Some(ref r) => r.quota_used,
            _ => 0.into(),
        };

        let no_of_logs = receipts.iter().fold(0, |acc, r| acc + r.logs.len());

        if last_receipt.transaction_hash == id {
            // Get sender
            let stx = self.transaction_by_address(hash, index).unwrap();
            let number = self.block_height_by_hash(hash).unwrap_or(0);

            let contract_address = match *stx.action() {
                Action::Create if last_receipt.error.is_none() => {
                    Some(core::libchain::chain::contract_address(stx.sender(), &last_receipt.account_nonce))
                }
                _ => None,
            };

            let receipt = LocalizedReceipt {
                transaction_hash: id,
                transaction_index: index,
                block_hash: hash,
                block_number: number,
                cumulative_quota_used: last_receipt.quota_used,
                quota_used: last_receipt.quota_used - prior_quota_used,
                contract_address,
                logs: last_receipt
                    .logs
                    .into_iter()
                    .enumerate()
                    .map(|(i, log)| LocalizedLogEntry {
                        entry: log,
                        block_hash: hash,
                        block_number: number,
                        transaction_hash: id,
                        transaction_index: index,
                        transaction_log_index: i,
                        log_index: no_of_logs + i,
                    })
                    .collect(),
                log_bloom: last_receipt.log_bloom,
                state_root: last_receipt.state_root,
                error: last_receipt.error,
            };
            Some(receipt)
        } else {
            error!("The transaction_hash in receipt is not equal to transaction hash from input.");
            None
        }
    }


    /// Get transaction by address
    fn transaction_by_address(&self, hash: H256, index: usize) -> Option<SignedTransaction> {
        self.block_body_by_hash(hash)
            .map(|body| body.transactions()[index].clone())
    }

    /// Get block body by hash
    fn block_body_by_hash(&self, hash: H256) -> Option<BlockBody> {
        self.block_height_by_hash(hash)
            .and_then(|h| self.block_body_by_height(h))
    }

    fn block_header_by_height(&self, idx: BlockNumber) -> Option<Header> {
        let result ;
        match self.block_by_height(idx) {
            Some(block) => {
                result = Some(block.header().clone());
            }
            None => {
                result = None;
            }
        }
        result
    }

    // pub fn block_hash_by_height(&self, height: BlockNumber) -> Option<H256> {
    //     self.block_header_by_height(height)
    //         .and_then(|hdr| Some(hdr.hash().unwrap()))
    // }

    /// Get block body by height
    fn block_body_by_height(&self, number: BlockNumber) -> Option<BlockBody> {
        let result ;
        match self.block_by_height(number) {
            Some(block) => {
                result = Some(block.body().clone());
            }
            None => {
                result = None;
            }
        }
        result
    }

    /// Get block number by BlockId
    // fn block_number(&self, id: BlockId) -> Option<BlockNumber> {
    //     match id {
    //         BlockId::Number(number) => Some(number),
    //         BlockId::Hash(hash) => self.block_height_by_hash(hash),
    //         BlockId::Earliest => Some(0),
    //         BlockId::Latest => Some(self.get_latest_height()),
    //         BlockId::Pending => Some(self.get_pending_height()),
    //     }
    // }

    // pub fn logs<F>(
    //     &self,
    //     mut blocks: Vec<BlockNumber>,
    //     matches: F,
    //     limit: Option<usize>,
    // ) -> Vec<LocalizedLogEntry>
    // where
    //     F: Fn(&LogEntry) -> bool,
    //     Self: Sized,
    // {
    //     // sort in reverse order
    //     blocks.sort_by(|a, b| b.cmp(a));

    //     let mut log_index = 0;
    //     let mut logs = blocks
    //         .into_iter()
    //         .filter_map(|number| self.block_hash_by_height(number).map(|hash| (number, hash)))
    //         .filter_map(|(number, hash)| {
    //             self.block_receipts(hash)
    //                 .map(|r| (number, hash, r.receipts))
    //         })
    //         .filter_map(|(number, hash, receipts)| {
    //             self.block_body_by_hash(hash)
    //                 .map(|ref b| (number, hash, receipts, b.transaction_hashes()))
    //         })
    //         .flat_map(|(number, hash, mut receipts, mut hashes)| {
    //             if receipts.len() != hashes.len() {
    //                 warn!(
    //                     "Block {} ({}) has different number of receipts ({}) to transactions ({}). Database corrupt?",
    //                     number,
    //                     hash,
    //                     receipts.len(),
    //                     hashes.len()
    //                 );
    //                 unreachable!();
    //             }
    //             log_index = receipts
    //                 .iter()
    //                 .fold(0, |sum, receipt| sum + receipt.logs.len());

    //             let receipts_len = receipts.len();
    //             hashes.reverse();
    //             receipts.reverse();
    //             receipts
    //                 .into_iter()
    //                 .map(|receipt| receipt.logs)
    //                 .zip(hashes)
    //                 .enumerate()
    //                 .flat_map(move |(index, (mut logs, tx_hash))| {
    //                     let current_log_index = log_index;
    //                     let no_of_logs = logs.len();
    //                     log_index -= no_of_logs;

    //                     logs.reverse();
    //                     logs.into_iter().enumerate().map(move |(i, log)| {
    //                         LocalizedLogEntry {
    //                             entry: log,
    //                             block_hash: hash,
    //                             block_number: number,
    //                             transaction_hash: tx_hash,
    //                             // iterating in reverse order
    //                             transaction_index: receipts_len - index - 1,
    //                             transaction_log_index: no_of_logs - i - 1,
    //                             log_index: current_log_index - i - 1,
    //                         }
    //                     })
    //                 })
    //         })
    //         .filter(|log_entry| matches(&log_entry.entry))
    //         .take(limit.unwrap_or(::std::usize::MAX))
    //         .collect::<Vec<LocalizedLogEntry>>();
    //     logs.reverse();
    //     logs
    // }

    // /// Returns numbers of blocks containing given bloom.
    // pub fn blocks_with_bloom(
    //     &self,
    //     bloom: &LogBloom,
    //     from_block: BlockNumber,
    //     to_block: BlockNumber,
    // ) -> Vec<BlockNumber> {
    //     let range = from_block as BloomChainNumber..to_block as BloomChainNumber;
    //     BloomGroupChain::new(self.blooms_config, self)
    //         .with_bloom(&range, &Bloom::from(Into::<[u8; 256]>::into(*bloom)))
    //         .into_iter()
    //         .map(|b| b as BlockNumber)
    //         .collect()
    // }

    // /// Returns numbers of blocks containing given bloom by blockId.
    // pub fn blocks_with_bloom_by_id(
    //     &self,
    //     bloom: &LogBloom,
    //     from_block: BlockId,
    //     to_block: BlockId,
    // ) -> Option<Vec<BlockNumber>> {
    //     match (
    //         self.block_number(from_block),
    //         self.block_number(to_block),
    //         self.block_number(BlockId::Pending),
    //     ) {
    //         (Some(from), Some(to), Some(pending)) => {
    //             let end = if to > pending { pending } else { to };
    //             Some(self.blocks_with_bloom(bloom, from, end))
    //         }
    //         _ => None,
    //     }
    // }

    // pub fn get_logs(&self, filter: &Filter) -> Vec<LocalizedLogEntry> {
    //     let blocks = filter
    //         .bloom_possibilities()
    //         .iter()
    //         .filter_map(|bloom| {
    //             self.blocks_with_bloom_by_id(bloom, filter.from_block, filter.to_block)
    //         })
    //         .flat_map(|m| m)
    //         // remove duplicate elements
    //         .collect::<HashSet<u64>>()
    //         .into_iter()
    //         .collect::<Vec<u64>>();

    //     self.logs(blocks, |entry| filter.matches(entry), filter.limit)
    // }

    pub fn get_block_header_bytes(&self, id: BlockId) -> Option<Vec<u8>> {
        self.block_header(id).map(|x| x.rlp_bytes().into_vec())
    }

    #[inline]
    pub fn get_old_height(&self) -> u64 {
        self.old_height.load(Ordering::SeqCst) as u64
    }

    #[inline]
    pub fn get_sectionid(&self) -> u64 {
        self.sectionid.load(Ordering::SeqCst) as u64
    }

    #[inline]
    pub fn set_old_height(&self, old_height: u64) {
        self.old_height
                        .store(old_height as usize, Ordering::SeqCst);
        // info!("block_encoder.old_height = {} ", self.old_height.load(Ordering::SeqCst));
    }

    #[inline]
    pub fn set_sectionid(&self, sectionid:u64) {
        self.sectionid
                .store(sectionid as usize, Ordering::SeqCst);
        // info!("block_encoder.sectionid = {} ", self.sectionid.load(Ordering::SeqCst));
    }

    #[inline]
    pub fn set_oldn(&self, n:u64) {
        self.oldn.store(n as usize, Ordering::SeqCst);
    }

    #[inline]
    pub fn get_oldn(&self) -> u64 {
        self.oldn.load(Ordering::SeqCst) as u64
    }

    #[inline]
    pub fn get_pending_height(&self) -> u64 {
        0
    }

    #[inline]
    pub fn get_latest_height(&self) -> u64 {
        0
    }

    pub fn get_datablockmap(&self, datablockid: String) ->  Option<Vec<u8>> {
        let result = self
            .mydb
            .read_with_cache(mydb::MYCOL_DATABLOCKS, &self.datablockmap, &datablockid);
        result
        // 标记被使用 TODO
        // self.datablockmap.read().clone()
    }

    pub fn get_datablocksize(&self, datablockid: String) -> Option<usize> {
        let result = self
            .mydb
            .read_with_cache(mydb::MYCOL_SIZE, &self.datablocksizemap, &datablockid);
        result
    }

    pub fn get_maxblocknodesize(&self, sectionid: u64) -> Option<usize> {
        let result = self
            .mydb
            .read_with_cache(mydb::MYCOL_MAXSIZE, &self.maxdatablocksizemap, &sectionid);
        result
    }

    // pub fn set_datablockmap(&self, datablockmap: HashMap<String, Vec<u8>>) {
    //     for (key, value) in datablockmap {
    //         self.datablockmap.write().insert(key, value);
    //     }
    //     // info!("block_encoder.datablockmap = {:?} ", self.datablockmap.read().clone().keys());
    // }

    pub fn get_kmap(&self, sectionid: u64) -> Option<u64> {
        let result = self
            .mydb
            .read_with_cache(mydb::MYCOL_K, &self.kmap, &sectionid);
        // 上锁 TODO
        result
    }

    // pub fn insert_kmap(&self, key: u64, value: u64) {
    //     self.kmap.write().insert(key, value);
    // }

    pub fn get_mmap(&self, sectionid: u64) -> Option<u64> {
        let result = self
            .mydb
            .read_with_cache(mydb::MYCOL_M, &self.mmap, &sectionid);
        // 上锁 TODO
        result
    }

    // pub fn insert_mmap(&self, key: u64, value: u64) {
    //     self.mmap.write().insert(key, value);
    // }

    // pub fn get_addr_peer(&self) -> HashMap<H160, PeerConfig> {
    //     self.addr_peer.read().clone()
    // }

    pub fn set_addr_peer(&self, peers:HashMap<H160, PeerConfig>) {
        self.addr_peer.write().clear();
        for (key, value) in peers {
            self.addr_peer.write().insert(key, value);
        }
        // info!("set_addr_peer:{:?}", self.addr_peer.read().clone());
    }

    pub fn get_allnodes(&self)->ConsensusNodes {
        self.allnodes.read().clone()
    }

    pub fn set_allnodes(&self, nodes: ConsensusNodes) {
        let mut newnodes =  self.allnodes.read().nodes.clone();
        let allnodes = &self.allnodes.read().nodes.clone();
        let mut flag:bool  = false;
        for node in nodes.nodes {
            for node1 in allnodes {
                // info!("node: {:?}, node1: {:?}", node, *node1);
                if node == *node1 {
                    flag = true;
                }
            }
            if flag == false {
                newnodes.push(node);
            }
            flag = false;
        }
        self.allnodes.write().nodes.clone_from(&newnodes);

        // 存数据库
        if allnodes.len() != newnodes.len() {
            let mut batch = DBTransaction::new();
            let nodes = ConsensusNodes::new(newnodes.clone());
            batch.write(mydb::MYCOL_EXTRA, &AllNodes, &nodes);
            self.mydb.write(batch).expect("DB write failed.");
            info!("set_allnodes:{:?}", self.allnodes.read().clone());
        }
    }


    // pub fn get_allnodes(&self) -> Vec<Address> {
    //     self.allnodes.read().nodes.clone()
    // }

    // pub fn set_allnodes(&self, nodes: Vec<Address>) {
    //     let newnodes = nodes.clone();
    //     let allnodes = self.allnodes.read().nodes;
    //     for node in allnodes {
    //         for node1 in nodes {
    //             if node == node1 {
    //                 newnodes.push(node);
    //             }
    //         }
    //     }

    //     self.allnodes.write().nodes.clean();
    //     self.allnodes.write().nodes.clone_from(newnodes);

    //     let mut batch = DBTransaction::new();
    //     let nodes = ConsensusNodes::new(newnodes.clone());
    //     batch.write(mydb::MYCOL_EXTRA, &AllNodes, &nodes);
    //     self.mydb.write(batch).expect("DB write failed.");
    //     info!("set_allnodes:{:?}", self.allnodes.read().clone());
    // }

    // pub fn datablockmap_insert(&self, datablockid: String, datablock: Vec<u8>) {
    //     self.datablockmap.write().insert(datablockid, datablock);
    //     info!("block_encoder.datablockmap = {:?} ", self.datablockmap.read().clone().keys());
    // }

    // pub fn get_datablocksizemap(&self) -> HashMap<String, usize> {
    //     self.datablocksizemap.read().clone()
    // }

    // pub fn set_datablocksizemap(&self, datablocksizemap: HashMap<String, usize>) {
    //     for (key, value) in datablocksizemap {
    //         self.datablocksizemap.write().insert(key, value);
    //     }
    //     // info!("block_encoder.datablocksizemap = {:?} ", self.datablocksizemap.read().clone().values());
    // }

    // pub fn datablocksizemap_insert(&self, datablockid: String, datablocksize: usize) {
    //     self.datablocksizemap.write().insert(datablockid, datablocksize);
    // }

    pub fn get_blocknodesmap(&self, sectionid: u64) ->Option<ConsensusNodes> {
        let result = self
            .mydb
            .read_with_cache(mydb::MYCOL_CONSENSUS_NODES, &self.block_nodes_map, &sectionid);
        result
        // TODO:标记被使用
        // self.block_nodes_map.read().clone()
    }

    // pub fn blocknodesmap_insert(&self, datablockid:String, blocknodes: ConsensusNodes) {
    //     self.block_nodes_map.write().insert(datablockid, blocknodes);
    // }

    // pub fn get_hash_height_map(&self) -> HashMap<H256, BlockNumber> {
    //     self.hash_height_map.read().clone()
    // }

    // pub fn set_hash_height_map(&self, map: HashMap<H256,BlockNumber>) {
    //     for (key, value) in map {
    //         self.hash_height_map.write().insert(key, value);
    //     }
    //     // info!("hash_height_map = {:?}", self.hash_height_map.read().clone().keys());
    // }

    pub fn get_transaction_addresses(&self) -> HashMap<TransactionId, TransactionAddress> {
        self.transaction_addresses.read().clone()
    }

    // pub fn set_transaction_addresses(&self, addrs: HashMap<TransactionId, TransactionAddress>) {
    //     for (key, value) in addrs {
    //         self.transaction_addresses.write().insert(key, value);
    //     }
    // }

    pub fn get_block_receipts(&self) ->HashMap<H256, BlockReceipts> {
        self.block_receipts.read().clone()
    }

    // pub fn set_block_receipts(&self, receipts:HashMap<H256, BlockReceipts>) {
    //     for (key, value) in receipts {
    //         self.block_receipts.write().insert(key, value);
    //     }
    // }

}

// #[derive(Debug, Deserialize, Clone)]
// pub struct UpStream {
//     pub url: String,
//     pub timeout: Duration,
// }

// impl UpStream {
//     pub fn new(url: String, timeout: Duration) -> UpStream {
//         UpStream{url, timeout}
//     }
// }

#[derive(Debug, Deserialize, Clone)]
pub struct NetConfig {
    pub port: Option<usize>,
    pub peers: Option<Vec<PeerConfig>>,
    pub max_connects: Option<usize>,
    pub enable_tls: Option<bool>,
    pub enable_discovery: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PeerConfig {
    pub ip: Option<String>,
    pub port: Option<usize>,
}

impl PeerConfig {
    // pub fn new(ip: String, port: usize) -> Self {
    //     let peer = PeerConfig {
    //         Some(ip),
    //         Some(port),
    //     };
    //     peer
    // }

    pub fn new(ip: String, port: String) -> Self {
        let peer = PeerConfig {
            ip: Some(ip),
           port: Some(port.parse::<usize>().unwrap() + 1337 - 4000),
        };
        peer
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Post {
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplyType {
    pub jsonrpc: Option<rpc_types::Version>,
    pub id: rpc_types::Id,
    pub result: JsonBlock,
}

// impl Option {
//     pub fn ok_or(self, err: E) -> Result 
//     {
//         match self {
//             Some(v) => Ok(v),
//             None => Err(err),
//         }
//     }
// }
