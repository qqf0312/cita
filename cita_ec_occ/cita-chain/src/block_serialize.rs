extern crate serde_json;
extern crate serde;

use crate::ffi_wrapper::encoder_clothes_w;
// use crate::ffi_wrapper::decoder_clothes_w;

use crate::serializedblock::SerializedBlock;
use crate::datablock::DataBlock;
use crate::block_encoder::BlockEncoder;
// use libproto::TryInto;
// use libproto::TryFrom;
// use libproto::blockchain::Block as ProtoBlock;
// use jsonrpc_types::rpc_types::RpcBlock;

use core::libchain::chain::Chain;
use types::block::Block;
use std::sync::Arc;
use std::collections::HashMap;
use cita_types::H256;
use types::BlockNumber;
use rlp::*;
use crate::types::extras::*;
use crate::types::ids::TransactionId;
// use libproto::blockchain::{Proof as ProtoProof, ProofType};
// use proof::BftProof;

// use types::crypto::Signature;
// use cita_types::Address;
// use jsonrpc_types::rpc_request;
use jsonrpc_types::rpc_types;
// use crate::block_encoder::RpcClient;
// use crate::block_encoder::Error;
// use crate::block_encoder::PeerConfig;
// use crate::block_encoder::UpStream;

use serde::Serialize;
use serde::Deserialize;
// use std::time::Duration;
// use std::str::FromStr;

// use crate::types::log_blooms::LogBloomGroup;

/// Processing blocks and transaction storage
#[derive(Clone)]
pub struct BlockSerialize {
    pub chain: Arc<Chain>,
    pub blockencoder: Arc<BlockEncoder>,
}

impl BlockSerialize {
    pub fn new(chain: Arc<Chain>, blockencoder: Arc<BlockEncoder>) -> Self {
        BlockSerialize {chain, blockencoder}
    }

    pub fn get_chain_height(&self) ->u64 {
        self.chain.get_latest_height()
    }

    pub fn get_old_height(&self) ->u64 {
        self.blockencoder.get_old_height()
    }

    pub fn getblock(&self, blockid:u64) -> Option<Block> {
        self.chain.block_by_height(blockid)
    }

    // 纠删码编码
    pub fn encoder(&self) {
        // self.chain.encoder(section_size, k, m, w);
        let old_height = self.get_old_height(); //上一次编码后的高度
        let newcurrent = self.get_chain_height(); //目前区块高度
        let section_size = self.blockencoder.section_size;
        // info!("old_height={}, newcurrent={}", old_height, newcurrent);

        //如果 新增的区块高大于区间大小section_size
        if newcurrent - old_height > section_size {
            // let mut datablockmap:Vec<String> = Vec::new();  //数据块数组
            // 区间最后一个区块的共识节点公钥
            let addresses = self.chain.get_block_consensus_nodes(old_height + section_size - 1).unwrap().clone();
            let n = addresses.nodes.len() as u64;
            let m; // m

            // 计算该区间冗余块个数
            if (n - 1) % 3 == 0 {
                m = (n - 1) / 3;
            } else {
                m = (n - 1) / 3 + 1;
            }
            let k = n - m; //k
            let w = self.blockencoder.w; //w
            let sectionid:u64 = self.blockencoder.get_sectionid(); //区间id
            info!("sectionid = {}", sectionid);
            let oldk;
            let oldm;
            let oldn = self.blockencoder.get_oldn();
            if sectionid >0 {
                oldk = self.blockencoder.get_kmap(sectionid - 1).unwrap().clone(); //上一区间k
                oldm = self.blockencoder.get_mmap(sectionid - 1).unwrap().clone(); //上一区间m
            } else {
                oldk = k;
                oldm = m;
            }
            info!("k: {:?}, m: {:?}, oldk: {:?}, oldm: {:?}", k, m, oldk, oldm);

            // 判断 扩缩容
            if n != oldn {
                // // 获取其他节点的Address和IP:Port
                // let peerconf = PeerConfig::new("127.0.0.1".to_string(), self.blockencoder.port.to_string());
                // let upstream = UpStream::new(String::from(format!("http://{}:{}", peerconf.clone().ip.unwrap(), peerconf.clone().port.unwrap())) , Duration::new(100, 0));
                // let req = rpc_request::PeersInfoParams::new().into_request(1);
                // info!("req = {:?}, uri = {:?}", req, format!("http://{}:{}", "127.0.0.1", self.blockencoder.port));
                // let rpc_cli = RpcClient::create(&upstream);
                // let body: String = req.into();
                // let data = rpc_cli.do_post(&body);
                // // info!("data = {:?}", data);
                // let reply = match data {
                //     Ok(chunk) => {
                //         let slicereply = chunk.as_ref();
                //         info!("slicereply = {:?}", slicereply);
                //         // let replyvec = slicereply.to_vec();
                //         // let strreply = String::from_utf8(replyvec).unwrap();
                //         // let strreply = strreply.as_str();
                //         // info!("strreply = {:?}", strreply);
                //         match serde_json::from_slice::<PeersReply>(slicereply) {
                //             Ok(reply) => {
                //                 Ok(reply)
                //             }
                //             Err(e) => {
                //                 error!("parse error :{:?}", e);
                //                 Err(Error::Parse)
                //             }
                //         }
                //         // info!("reply = {:?}", reply);
                //     }
                //     Err(e) => {
                //         error!("err = {:?}", e);
                //         Err(e)
                //     }
                // };
                // let peersinfo = reply.expect("get peer info error").result;
                // let peers = peersinfo.peers.unwrap();
                // let mut addr_peer:HashMap<Address, PeerConfig> = HashMap::new();
                // for (key, value) in peers {
                //     let ipport: String = value;
                //     let strpeer: Vec<&str> = ipport.split(':').collect();
                //     let peerconf = PeerConfig::new(String::from_str(strpeer.get(0).unwrap()).unwrap(), String::from_str(strpeer.get(1).unwrap()).unwrap());
                //     addr_peer.insert(key, peerconf);
                // }
                // // 将自己也加进去
                // addr_peer.insert(self.blockencoder.address, PeerConfig::new("127.0.0.1".to_string(), self.blockencoder.port.to_string()));
                // self.blockencoder.set_addr_peer(addr_peer);

                if n > oldn { //扩容
                    if m == oldm { //不需处理历史编码块

                    } else {

                    }
                } else {  // 缩容
                    if m == oldm - 1 { //不需处理历史编码块

                    } else {

                    }
                } 
            } else { // 不变


            }



            // self.blockencoder.insert_kmap(sectionid, k);
            // self.blockencoder.insert_mmap(sectionid, m);
            
            let mut datablocksize; //数据块大小 中间变量
            let mut maxdatablocksize= 0; //最大的数据块大小
            let mut blocku8:Vec<u8> = Vec::new();  //u8类型 一维数据块 传入参数
            let mut datablocksizemap:HashMap<String, usize> = HashMap::new(); //数据块大小数组
            let mut datablockmap:HashMap<String, Vec<u8>> = HashMap::new();  //数据块数组
            let mut hash_height_map: HashMap<H256, BlockNumber> = HashMap::new();  //hash_height映射
            let mut transaction_addresses:HashMap<TransactionId, TransactionAddress> = HashMap::new();  // 交易id与交易地址的map映射
            let mut block_receipts:HashMap<H256, BlockReceipts> = HashMap::new();
            
            // 数据块能由区块均分得到的情况
            if section_size % k == 0 {
                let mut block_height = old_height;  //获取区块的索引——数据块高度
                for _i in 0..k {
                    // info!("i = {}", _i);
                    let mut datablock:Vec<SerializedBlock> = Vec::new();  //数据块
                    for _j in 0..section_size / k{
                        // info!("j = {}", _j);
                        // info!("block_height:{}", block_height);
                        // 获得区块 并序列化区块
                        match self.getblock(block_height) {
                            Some(block) => {  //得到一个区块
                                if block.body().transactions.len() > 0 {
                                    info!("Has Transactions Block: {:?}", block.body().transactions.len());
                                }
                                // 存储区块hash与height映射
                                let hash = block.hash().unwrap();
                                hash_height_map.insert(hash, block_height);
                                transaction_addresses =  block.body().transaction_addresses(hash);
                                self.chain.block_receipts(hash).and_then(|receipts| block_receipts.insert(hash, receipts) );

                                // let proof = block.header().proof().clone();
                                // info!("proof = {:?}", proof);
                                // if block_height > 0 { 
                                //     let bft_proof = BftProof::from(proof);
                                //     let mut proof_addresses:Vec<Address> = Vec::new();
                                //     for address in bft_proof.commits.keys() {
                                //         proof_addresses.push(*address);
                                //     }
                                //     info!("proof_addresses = {:?}", proof_addresses);
                                // }
                                
                                // 序列化
                                let mut rlp = RlpStream::new();
                                block.rlp_append(&mut rlp);
                                let bufblock = rlp.out();
                                let serializedblock = SerializedBlock::new(block_height, bufblock);
                                datablock.push(serializedblock);
                            }
                            None => { //没得到
                                info!("Serialized Error{}", block_height);
                            }
                        }
                        block_height += 1;  //区块高度索引+1
                    } // 至此 一个数据块已经形成

                    // 序列化数据块
                    let datablock_withid = DataBlock::new(_i, datablock);
                    let mut serial = serde_json::to_string(&datablock_withid).unwrap();
                    // info!("one serial datablock= {}", serial);
                    datablocksize = serial.len();  //获取数据块 真实长度
                    unsafe {
                        datablockmap.insert(format!("{0}_{1}", sectionid, _i), serial.as_mut_vec().to_vec());  //序列化好的数据块放入数据块vec
                    }
                    // info!("insert_key = {}", format!("{0}_{1}", sectionid, _i));
                    datablocksizemap.insert(format!("{0}_{1}", sectionid, _i), datablocksize); //数据块真实长度放入vec
                    if maxdatablocksize < datablocksize {  //设置max
                        maxdatablocksize = datablocksize;
                    }
                }
            } else {  //不能均分
                let mut block_height = old_height;  //获取区块的索引——数据块高度
                for _i in 0..k {
                    // info!("i = {}", _i);
                    let mut datablock:Vec<SerializedBlock> = Vec::new();  //数据块
                    if _i < section_size % k {
                        for _j in 0..section_size / k + 1 {
                            // 获得区块 并序列化区块
                            match self.getblock(block_height) {
                                Some(block) => {  //得到一个区块
                                    // 打印交易
                                    if block.body().transactions.len() > 0 {
                                        info!("Has Transactions Block: {:?}", block.body().transactions.len());
                                    }
                                    // 存储区块hash与height映射
                                    let hash = block.hash().unwrap();
                                    hash_height_map.insert(hash, block_height);
                                    transaction_addresses =  block.body().transaction_addresses(hash);
                                    self.chain.block_receipts(hash).and_then(|receipts| block_receipts.insert(hash, receipts) );

                                    // let proof = block.header().proof().clone();
                                    // // info!("proof = {:?}", proof);
                                    // if block_height > 0 { 
                                    //     let bft_proof = BftProof::from(proof);
                                    //     let mut proof_addresses:Vec<Address> = Vec::new();
                                    //     for address in bft_proof.commits.keys() {
                                    //         proof_addresses.push(*address);
                                    //     }
                                    //     info!("proof_addresses = {:?}", proof_addresses);
                                    // }
                                    
                                    // 序列化
                                    let mut rlp = RlpStream::new();
                                    block.rlp_append(&mut rlp);
                                    let bufblock = rlp.out();
                                    let serializedblock = SerializedBlock::new(block_height, bufblock);
                                    datablock.push(serializedblock);
                                }
                                None => { //没得到
                                    info!("Serialized Error{}", block_height);
                                }
                            }
                            block_height += 1;  //区块高度索引+1
                            // 至此 一个数据块已经形成
                        }
                    } else {
                        for _j in 0..section_size / k {
                            // 获得区块 并序列化区块
                            match self.getblock(block_height) {
                                Some(block) => {  //得到一个区块
                                    if block.body().transactions.len() > 0 {
                                        info!("Has Transactions Block: {:?}", block.body().transactions.len());
                                    }
                                    // 存储区块hash与height映射
                                    let hash = block.hash().unwrap();
                                    hash_height_map.insert(hash, block_height);
                                    transaction_addresses =  block.body().transaction_addresses(hash);
                                    self.chain.block_receipts(hash).and_then(|receipts| block_receipts.insert(hash, receipts) );

                                    // let proof = block.header().proof().clone();
                                    // info!("proof = {:?}", proof);
                                    // if block_height > 0 { 
                                    //     let bft_proof = BftProof::from(proof);
                                    //     let mut proof_addresses:Vec<Address> = Vec::new();
                                    //     for address in bft_proof.commits.keys() {
                                    //         proof_addresses.push(*address);
                                    //     }
                                    //     info!("proof_addresses = {:?}", proof_addresses);
                                    // }
                                    
                                    // 序列化
                                    let mut rlp = RlpStream::new();
                                    block.rlp_append(&mut rlp);
                                    let bufblock = rlp.out();
                                    let serializedblock = SerializedBlock::new(block_height, bufblock);
                                    datablock.push(serializedblock);
                                }
                                None => { //没得到
                                    info!("Serialized Error{}", block_height);
                                }
                            }
                            block_height += 1;  //区块高度索引+1
                            // 至此 一个数据块已经形成
                        }
                    }

                    // 序列化数据块
                    let datablock_withid = DataBlock::new(_i, datablock);
                    let mut serial = serde_json::to_string(&datablock_withid).unwrap();
                    // info!("one serial datablock= {}", serial);
                    datablocksize = serial.len();  //获取数据块 真实长度
                    unsafe {
                        datablockmap.insert(format!("{0}_{1}", sectionid, _i), serial.as_mut_vec().to_vec());  //序列化好的数据块放入数据块vec
                    }
                    // info!("insert_key = {}", format!("{0}_{1}", sectionid, _i));
                    datablocksizemap.insert(format!("{0}_{1}", sectionid, _i), datablocksize); //数据块真实长度放入vec
                    if maxdatablocksize < datablocksize {  //设置max
                        maxdatablocksize = datablocksize;
                    }
                }
            }
            info!("datablocksizemap = {:?}", datablocksizemap.values());
            // info!("maxdatablocksize  = {}", maxdatablocksize);
            // 数据块长度必须是sizeof（long）=8的倍数
            while maxdatablocksize % 8 != 0 {
                maxdatablocksize += 1;
            }
            info!("maxdatablocksize  = {}", maxdatablocksize);

            // 形成一维数据块
            for (key, mut value) in datablockmap.clone() {
                // info!("key = {}", key);
                blocku8.append(&mut value);
                let mut j = *datablocksizemap.get(&key).unwrap();
                while j < maxdatablocksize {
                    blocku8.push(0x00);
                    j += 1;
               }
            }

            // info!("the latest datablock= {:?}", blocku8);

            // 格式转换 形成参数
            let raw_datablocksize = maxdatablocksize;
            let puncode_stream = blocku8.as_mut_ptr();
            let mut raw_coding:Vec<u8> = Vec::with_capacity(maxdatablocksize * (m as usize));
            let praw_coding = raw_coding.as_mut_ptr();

            // info!("格式转换ok！sectionid = {}", sectionid);

            // 编码
            encoder_clothes_w(k, m, w, puncode_stream, praw_coding , raw_datablocksize);

            // info!("raw_coding = {:?}", raw_coding);

            unsafe {
                let coding = make_slice(praw_coding, maxdatablocksize * m as usize);
                raw_coding = coding.to_vec();
                // core::slice::from_raw_parts_mut(praw_coding, (maxdatablocksize * m) as usize);
                // praw_coding.
                // .to_string().as_mut_vec().to_vec();
            }

            // 分割校验块
            for i in 0..m - 1 {
                // info!("raw_codingsize = {}, maxdatablocksize = {}", raw_coding.len(), maxdatablocksize);
                let vec = raw_coding.split_off(maxdatablocksize);
                datablockmap.insert(format!("{0}_{1}", sectionid, k+i), vec);
                datablocksizemap.insert(format!("{0}_{1}", sectionid, k+i), maxdatablocksize);
            }
            datablockmap.insert(format!("{0}_{1}", sectionid, k + m - 1), raw_coding);
            datablocksizemap.insert(format!("{0}_{1}", sectionid, k + m - 1), maxdatablocksize);
            
            // info!("数据块大小vec{:?}", datablocksizemap.values());

            let mut res_datablock = Vec::new();
            let mut res_datablocksize = 0;
            let mut res_datablockid = String::new();
            // 保存区间最后一个区块的共识节点公钥
            // old_height需要存数据库，跟着变化
            
            for i in 0..addresses.nodes.len() {
                if self.blockencoder.address == addresses.nodes.get(i).unwrap().clone() {
                    info!("this is the {:?} datablock", i);
                    res_datablock = datablockmap.get(&format!("{0}_{1}", sectionid, i)).unwrap().clone();
                    res_datablocksize = datablocksizemap.get(&format!("{0}_{1}", sectionid, i)).unwrap().clone();
                    res_datablockid = format!("{0}_{1}", sectionid, i);
                    break;
                }
            }

            self.blockencoder.set_encoder_result(sectionid, 
                                                old_height + section_size,
                                                n,
                                                k,
                                                m,
                                                res_datablockid,
                                                res_datablock,
                                                res_datablocksize,
                                                maxdatablocksize,
                                                // datablockmap,
                                                // datablocksizemap,
                                                addresses,
                                                hash_height_map, 
                                                transaction_addresses,
                                                block_receipts,
                                                self.chain.get_nodes());
        }
    }
}


unsafe fn make_slice<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    // place pointer address and length in contiguous memory
    let x: [usize; 2] = [ptr as usize, len];

    // cast pointer to array as pointer to slice
    let slice_ptr = &x as * const _ as *const &[u8];

    // dereference pointer to slice, so we get a slice
    *slice_ptr
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeersReply {
    pub jsonrpc: Option<rpc_types::Version>,
    pub id: rpc_types::Id,
    pub result: rpc_types::PeersInfo,
}