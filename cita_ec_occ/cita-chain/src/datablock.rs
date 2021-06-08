extern crate serde_json;
use serde::{Serialize, Deserialize};
use crate::serializedblock::SerializedBlock;

// 数据块
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct DataBlock {
    pub id: u64,  // 数据块id
    pub databody:Vec<SerializedBlock>,  //区块向量
}

impl DataBlock {
    pub fn new(id:u64, databody:Vec<SerializedBlock>) -> Self {
        DataBlock { id, databody }
    }
}