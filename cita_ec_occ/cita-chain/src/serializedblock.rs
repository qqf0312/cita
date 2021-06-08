extern crate serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SerializedBlock {
    pub id: u64, //区块高度
    pub blockbody:Vec<u8>,  //序列化后的区块
}

impl SerializedBlock {
    pub fn new(id:u64, blockbody:Vec<u8>) -> Self {
        SerializedBlock { id, blockbody}
    }
}