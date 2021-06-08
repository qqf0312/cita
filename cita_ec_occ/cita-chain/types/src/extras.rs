// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Blockchain DB extras.

use crate::basic_types::LogBloomGroup;
use crate::block::BlockBody;
use crate::db::Key;
use crate::mydb::Key as MyKey;
use crate::header::{BlockNumber, Header};
use crate::receipt::Receipt;
use bloomchain::group::GroupPosition;
use cita_types::{H256, H264,H160};
use libproto::blockchain::Proof;
use rlp::*;
use std::ops::{Deref, Index};
use util::*;

/// Represents index of extra data in database
#[derive(Copy, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExtrasIndex {
    /// Transaction address index
    TransactionAddress = 0,
    /// Block receipts index
    BlockReceipts = 1,
    /// Block blooms index
    BlocksBlooms = 2,
    /// Block hash index
    BlockHash = 3,
    /// Block head index
    BlockHeadHash = 4,
    /// Block body index
    BlockBodyHash = 5,
}

pub struct CurrentHash;

impl Key<H256> for CurrentHash {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f66")
    }
}

pub struct CurrentProof;

impl Key<Proof> for CurrentProof {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f67")
    }
}

pub struct CurrentHeight;

impl Key<BlockNumber> for CurrentHeight {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f68")
    }
}

pub struct OldHeight;

impl MyKey<BlockNumber> for OldHeight {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f69")
    }
}

pub struct SectionId;

impl MyKey<BlockNumber> for SectionId {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f70")
    }
}

pub struct OldN;

impl MyKey<BlockNumber> for OldN {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f71")
    }
}

pub struct AllNodes;

impl  Key<ConsensusNodes> for AllNodes {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f72")
    }
}

impl  MyKey<ConsensusNodes> for AllNodes {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f72")
    }
}

// impl MyKey<Vec<u8>> for String {
//     type  Target = Vec<u8>;
//     fn key(&self) -> Vec<u8> {
//         self.as_bytes().to_vec()
//     }
// }

// pub struct DataBlockIdKey(u64);

// impl Deref for DataBlockIdKey {
//     type Target = u64;

//     fn deref(&self) -> &Self::Target {
//         *self
//     }
// }

impl MyKey<usize> for String {
    type Target = DataBlockIdKey;
    fn key(&self) -> Self::Target {
        let datablockid:Vec<&str> = (*self).split('_').collect();
        let id = datablockid[0];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        let mut result = [0u8; 18];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (&id >> 56) as u8;
        result[2] = (&id >> 48) as u8;
        result[3] = (&id >> 40) as u8;
        result[4] = (&id >> 32) as u8;
        result[5] = (&id >> 24) as u8;
        result[6] = (&id >> 16) as u8;
        result[7] = (&id >> 8) as u8;
        result[8] = id as u8;

        let id = datablockid[1];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        // let mut result = [0u8; 9];
        result[9] = ExtrasIndex::BlockHeadHash as u8;
        result[10] = (&id >> 56) as u8;
        result[11] = (&id >> 48) as u8;
        result[12] = (&id >> 40) as u8;
        result[13] = (&id >> 32) as u8;
        result[14] = (&id >> 24) as u8;
        result[15] = (&id >> 16) as u8;
        result[16] = (&id >> 8) as u8;
        result[17] = id as u8;

        DataBlockIdKey(result)
    }
}

impl MyKey<Vec<u8>> for String {
    type Target = DataBlockIdKey;
    fn key(&self) -> Self::Target {
        let datablockid:Vec<&str> = (*self).split('_').collect();
        let id = datablockid[0];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        let mut result = [0u8; 18];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (&id >> 56) as u8;
        result[2] = (&id >> 48) as u8;
        result[3] = (&id >> 40) as u8;
        result[4] = (&id >> 32) as u8;
        result[5] = (&id >> 24) as u8;
        result[6] = (&id >> 16) as u8;
        result[7] = (&id >> 8) as u8;
        result[8] = id as u8;

        let id = datablockid[1];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        // let mut result = [0u8; 9];
        result[9] = ExtrasIndex::BlockHeadHash as u8;
        result[10] = (&id >> 56) as u8;
        result[11] = (&id >> 48) as u8;
        result[12] = (&id >> 40) as u8;
        result[13] = (&id >> 32) as u8;
        result[14] = (&id >> 24) as u8;
        result[15] = (&id >> 16) as u8;
        result[16] = (&id >> 8) as u8;
        result[17] = id as u8;
        DataBlockIdKey(result)
    }
}

impl MyKey<Vec<H160>> for String {
    type Target = DataBlockIdKey;
    fn key(&self) -> Self::Target {
        let datablockid:Vec<&str> = (*self).split('_').collect();
        let id = datablockid[0];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        let mut result = [0u8; 18];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (&id >> 56) as u8;
        result[2] = (&id >> 48) as u8;
        result[3] = (&id >> 40) as u8;
        result[4] = (&id >> 32) as u8;
        result[5] = (&id >> 24) as u8;
        result[6] = (&id >> 16) as u8;
        result[7] = (&id >> 8) as u8;
        result[8] = id as u8;

        let id = datablockid[1];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        // let mut result = [0u8; 9];
        result[9] = ExtrasIndex::BlockHeadHash as u8;
        result[10] = (&id >> 56) as u8;
        result[11] = (&id >> 48) as u8;
        result[12] = (&id >> 40) as u8;
        result[13] = (&id >> 32) as u8;
        result[14] = (&id >> 24) as u8;
        result[15] = (&id >> 16) as u8;
        result[16] = (&id >> 8) as u8;
        result[17] = id as u8;
        DataBlockIdKey(result)
    }
}

impl MyKey<ConsensusNodes> for String {
    type Target = DataBlockIdKey;
    fn key(&self) -> Self::Target {
        let datablockid:Vec<&str> = (*self).split('_').collect();
        let id = datablockid[0];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        let mut result = [0u8; 18];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (&id >> 56) as u8;
        result[2] = (&id >> 48) as u8;
        result[3] = (&id >> 40) as u8;
        result[4] = (&id >> 32) as u8;
        result[5] = (&id >> 24) as u8;
        result[6] = (&id >> 16) as u8;
        result[7] = (&id >> 8) as u8;
        result[8] = id as u8;

        let id = datablockid[1];
        let id = id.to_string();
        let id:u64 = id.parse::<u64>().unwrap();
        // let mut result = [0u8; 9];
        result[9] = ExtrasIndex::BlockHeadHash as u8;
        result[10] = (&id >> 56) as u8;
        result[11] = (&id >> 48) as u8;
        result[12] = (&id >> 40) as u8;
        result[13] = (&id >> 32) as u8;
        result[14] = (&id >> 24) as u8;
        result[15] = (&id >> 16) as u8;
        result[16] = (&id >> 8) as u8;
        result[17] = id as u8;
        DataBlockIdKey(result)
    }
}

impl Key<ConsensusNodes> for u64 {
    type Target = BlockNumberKeyLong;
    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

impl MyKey<ConsensusNodes> for u64 {
    type Target = BlockNumberKeyLong;
    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

impl MyKey<u64> for u64 {
    type Target = BlockNumberKeyLong;
    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

impl MyKey<usize> for u64 {
    type Target = BlockNumberKeyLong;
    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

// impl MyKey<Vec<H160>> for String {
//     type Target = u64;
//     fn key(&self) -> u64 {
//         let datablockid:Vec<&str> = *self.split('_').collect();
//         let id = datablockid[0].as_bytes().clone() as u64;
//         id
//     }
// }

impl MyKey<u64> for H256 {
    type Target = H256;
    fn key(&self) -> H256 {
        *self
    }
}


impl MyKey<TransactionAddress> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

impl MyKey<BlockReceipts> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

impl Key<Header> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

impl Key<BlockBody> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

impl Key<BlockNumber> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

pub struct BlockNumberKey([u8; 5]);

impl Deref for BlockNumberKey {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DataBlockIdKey([u8; 18]);

impl Deref for DataBlockIdKey {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct BlockNumberKeyLong([u8; 9]);

impl Deref for BlockNumberKeyLong {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Key<Header> for BlockNumber {
    type Target = BlockNumberKeyLong;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

impl Key<BlockBody> for BlockNumber {
    type Target = BlockNumberKeyLong;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockBodyHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

impl Key<H256> for BlockNumber {
    type Target = BlockNumberKey;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 5];
        result[0] = ExtrasIndex::BlockHash as u8;
        result[1] = (self >> 24) as u8;
        result[2] = (self >> 16) as u8;
        result[3] = (self >> 8) as u8;
        result[4] = *self as u8;
        BlockNumberKey(result)
    }
}

fn with_index(hash: &H256, i: ExtrasIndex) -> H264 {
    let mut result = H264::default();
    result[0] = i as u8;
    (*result)[1..].clone_from_slice(hash);
    result
}

impl Key<TransactionAddress> for H256 {
    type Target = H264;

    fn key(&self) -> H264 {
        with_index(self, ExtrasIndex::TransactionAddress)
    }
}

impl Key<BlockReceipts> for H256 {
    type Target = H264;

    fn key(&self) -> H264 {
        with_index(self, ExtrasIndex::BlockReceipts)
    }
}

pub struct LogGroupKey([u8; 6]);

impl Deref for LogGroupKey {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LogGroupPosition(GroupPosition);

impl From<GroupPosition> for LogGroupPosition {
    fn from(position: GroupPosition) -> Self {
        LogGroupPosition(position)
    }
}

impl HeapSizeOf for LogGroupPosition {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl Key<LogBloomGroup> for LogGroupPosition {
    type Target = LogGroupKey;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 6];
        result[0] = ExtrasIndex::BlocksBlooms as u8;
        result[1] = self.0.level as u8;
        result[2] = (self.0.index >> 24) as u8;
        result[3] = (self.0.index >> 16) as u8;
        result[4] = (self.0.index >> 8) as u8;
        result[5] = self.0.index as u8;
        LogGroupKey(result)
    }
}

/// Represents address of certain transaction within block
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAddress {
    /// Block hash
    pub block_hash: H256,
    /// Transaction index within the block
    pub index: usize,
}

impl HeapSizeOf for TransactionAddress {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl Decodable for TransactionAddress {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        let tx_address = TransactionAddress {
            block_hash: rlp.val_at(0)?,
            index: rlp.val_at(1)?,
        };

        Ok(tx_address)
    }
}

impl Encodable for TransactionAddress {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.block_hash);
        s.append(&self.index);
    }
}

/// Contains all block receipts.
#[derive(Clone)]
pub struct BlockReceipts {
    pub receipts: Vec<Receipt>,
}

impl BlockReceipts {
    pub fn new(receipts: Vec<Receipt>) -> Self {
        BlockReceipts { receipts }
    }
}

impl Decodable for BlockReceipts {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(BlockReceipts {
            receipts: rlp.as_list()?,
        })
    }
}

impl Encodable for BlockReceipts {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.receipts);
    }
}

impl HeapSizeOf for BlockReceipts {
    fn heap_size_of_children(&self) -> usize {
        self.receipts.heap_size_of_children()
    }
}

impl Index<usize> for BlockReceipts {
    type Output = Receipt;
    fn index(&self, i: usize) -> &Receipt {
        &self.receipts[i]
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsensusNodes {
    pub nodes: Vec<H160>,
}

impl  ConsensusNodes {
    pub fn new(nodes: Vec<H160>) -> Self {
        ConsensusNodes{ nodes }
    }
}

impl Decodable for ConsensusNodes {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(ConsensusNodes {
            nodes: rlp.as_list()?,
        })
    }
}

impl Encodable for ConsensusNodes {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.nodes);
    }
}

impl HeapSizeOf for ConsensusNodes {
    fn heap_size_of_children(&self) -> usize {
        self.nodes.heap_size_of_children()
    }
}

impl Index<usize> for ConsensusNodes {
    type Output = H160;
    fn index(&self, i: usize) -> &H160 {
        &self.nodes[i]
    }
}

#[cfg(test)]
mod tests {
    use super::BlockReceipts;
    use rlp::*;

    #[test]
    fn encode_block_receipts() {
        let br = BlockReceipts::new(Vec::new());

        let mut s = RlpStream::new_list(2);
        s.append(&br);
        assert!(!s.is_finished(), "List shouldn't finished yet");
        s.append(&br);
        assert!(s.is_finished(), "List should be finished now");
        s.out();
    }
}
