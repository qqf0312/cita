extern crate serde_json;
use serde::{Serialize, Deserialize};
use cita_types::H160;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct EcNetMessage {
    pub node_address: H160, //目的节点公钥
    pub msg:Vec<u8>,  //序列化后的区块
}

impl EcNetMessage {
    pub fn new(node_address: H160, msg:Vec<u8>) -> Self {
        EcNetMessage { node_address, msg}
    }
}
