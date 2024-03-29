// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

use crate::Error as EvmError;
use crate::Ext;
//use cita_logger::log;
use cita_types::{H256, U256};
use std::boxed::Box;
use std::convert::From;
use std::string::FromUtf8Error;
use util::sha3;

////////////////////////////////////////////////////////////////////////////////
pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, EvmError>;
}
pub trait Deserialize: Sized {
    fn deserialize(bytes: &[u8]) -> Result<Self, EvmError>;
}

////////////////////////////////////////////////////////////////////////////////
impl Serialize for U256 {
    fn serialize(&self) -> Result<Vec<u8>, EvmError> {
        //let mut vec = Vec::with_capacity(64);
        let mut vec = vec![0; 32];
        self.to_big_endian(&mut vec);
        Ok(vec)
    }
}
impl Deserialize for U256 {
    fn deserialize(bytes: &[u8]) -> Result<Self, EvmError> {
        Ok(U256::from(bytes))
    }
}

////////////////////////////////////////////////////////////////////////////////
impl Serialize for String {
    fn serialize(&self) -> Result<Vec<u8>, EvmError> {
        Ok(self.to_owned().into_bytes())
    }
}
impl Deserialize for String {
    fn deserialize(bytes: &[u8]) -> Result<Self, EvmError> {
        Ok(Self::from_utf8(bytes.to_owned())?)
    }
}

////////////////////////////////////////////////////////////////////////////////
impl Serialize for Vec<u8> {
    fn serialize(&self) -> Result<Vec<u8>, EvmError> {
        Ok(self.clone())
    }
}
impl Deserialize for Vec<u8> {
    fn deserialize(bytes: &[u8]) -> Result<Self, EvmError> {
        Ok(Vec::from(bytes))
    }
}

////////////////////////////////////////////////////////////////////////////////
impl From<FromUtf8Error> for EvmError {
    fn from(err: FromUtf8Error) -> Self {
        EvmError::Internal(format!("Internal error: {}", err))
    }
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scalar {
    position: H256,
}

impl Scalar {
    pub fn new(position: H256) -> Self {
        Scalar { position }
    }
    // single element
    pub fn set(self: &Self, ext: &mut Ext, value: U256) -> Result<(), EvmError> {
        ext.set_storage(self.position, H256::from(value))?;
        Ok(())
    }

    pub fn get(self: &Self, ext: &mut Ext) -> Result<U256, EvmError> {
        let value = ext.storage_at_mut(&self.position)?;
        //info!("form evm fn get");
        Ok(U256::from(value))
    }

    // bytes & string
    pub fn set_bytes<T>(self: &Self, ext: &mut Ext, value: &T) -> Result<(), EvmError>
    where
        T: Serialize,
    {
        let encoded = value.serialize()?;
        let length = encoded.len();
        if length < 32 {
            let mut byte32 = [0u8; 32];
            byte32[0..encoded.len()].copy_from_slice(&encoded);
            byte32[31] = (length * 2) as u8;
            ext.set_storage(self.position, H256::from_slice(&byte32))?;
        } else {
            ext.set_storage(self.position, H256::from((length * 2 + 1) as u64))?;
            let mut key = U256::from(H256::from_slice(&sha3::keccak256(&self.position)));
            for chunk in encoded.chunks(32) {
                let value = H256::from(chunk);
                ext.set_storage(H256::from(key), value)?;
                key = key + U256::one();
            }
        }
        Ok(())
    }

    pub fn get_bytes<T>(self: &Self, ext: &mut Ext) -> Result<Box<T>, EvmError>
    where
        T: Deserialize,
    {
        let mut bytes = Vec::<u8>::new();
        let first = ext.storage_at_mut(&self.position)?;
        if first[31] % 2 == 0 {
            let len = (first[31] / 2) as usize;
            bytes.extend_from_slice(&first[0..len]);
            let decoded = T::deserialize(&bytes)?;
            Ok(Box::new(decoded))
        } else {
            let mut len = ((first.low_u64() as usize) - 1) / 2;
            let mut key = U256::from(H256::from_slice(&sha3::keccak256(&self.position)));
            let mut bytes = Vec::new();
            while len > 0 {
                let v = ext.storage_at_mut(&H256::from(key))?;
                if len > 32 {
                    bytes.extend_from_slice(v.as_ref());
                    key = key + U256::one();
                    len -= 32;
                } else {
                    bytes.extend_from_slice(&v[0..len]);
                    len = 0;
                }
            }
            Ok(Box::new(T::deserialize(&bytes)?))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array {
    position: H256,
}
impl Array {
    pub fn new(position: H256) -> Self {
        Array { position }
    }
    #[inline]
    fn key(&self, index: u64) -> H256 {
        let mut key = U256::from(H256::from_slice(&sha3::keccak256(&self.position)));
        key = key + U256::from(index);
        H256::from(key)
    }
    pub fn set(self: &Self, ext: &mut Ext, index: u64, value: &U256) -> Result<(), EvmError> {
        let scalar = Scalar::new(self.key(index));
        scalar.set(ext, *value)
    }

    pub fn get(self: &Self, ext: &mut Ext, index: u64) -> Result<U256, EvmError> {
        let scalar = Scalar::new(self.key(index));
        scalar.get(ext)
    }

    pub fn set_bytes<T>(self: &Self, ext: &mut Ext, index: u64, value: &T) -> Result<(), EvmError>
    where
        T: Serialize,
    {
        let scalar = Scalar::new(self.key(index));
        scalar.set_bytes(ext, value)
    }

    pub fn get_bytes<T>(self: &Self, ext: &mut Ext, index: u64) -> Result<Box<T>, EvmError>
    where
        T: Deserialize,
    {
        let scalar = Scalar::new(self.key(index));
        scalar.get_bytes(ext)
    }

    pub fn set_len(self: &Self, ext: &mut Ext, len: u64) -> Result<(), EvmError> {
        ext.set_storage(self.position, H256::from(len))?;
        Ok(())
    }

    pub fn get_len(self: &Self, ext: &Ext) -> Result<u64, EvmError> {
        let len = ext.storage_at(&self.position)?;
        Ok(len.low_u64())
    }

    pub fn get_array(self: &mut Self, index: u64) -> Array {
        Array::new(self.key(index))
    }
    pub fn get_map(self: &mut Self, index: u64) -> Map {
        Map::new(self.key(index))
    }
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Map {
    position: H256,
}

impl Map {
    pub fn new(position: H256) -> Self {
        Map { position }
    }
    #[inline]
    fn key<Key>(&self, key: &Key) -> Result<H256, EvmError>
    where
        Key: Serialize,
    {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&key.serialize()?);
        bytes.extend_from_slice(self.position.as_ref());
        Ok(H256::from_slice(&sha3::keccak256(&bytes)))
    }
    pub fn set<Key>(self: &Self, ext: &mut Ext, key: &Key, value: U256) -> Result<(), EvmError>
    where
        Key: Serialize,
    {
        Scalar::new(self.key(key)?).set(ext, value)
    }

    pub fn get<Key>(self: &Self, ext: &mut Ext, key: &Key) -> Result<U256, EvmError>
    where
        Key: Serialize,
    {
        Scalar::new(self.key(key)?).get(ext)
    }

    pub fn set_bytes<Key, Value>(
        self: &Self,
        ext: &mut Ext,
        key: &Key,
        value: &Value,
    ) -> Result<(), EvmError>
    where
        Key: Serialize,
        Value: Serialize,
    {
        Scalar::new(self.key(key)?).set_bytes(ext, value)
    }

    pub fn get_bytes<Key, Value>(self: &Self, ext: &mut Ext, key: &Key) -> Result<Value, EvmError>
    where
        Key: Serialize,
        Value: Deserialize,
    {
        Ok(*Scalar::new(self.key(key)?).get_bytes(ext)?)
    }

    pub fn get_array<Key>(self: &mut Self, key: &Key) -> Result<Array, EvmError>
    where
        Key: Serialize,
    {
        Ok(Array::new(self.key(key)?))
    }

    pub fn get_map<Key>(self: &mut Self, key: &Key) -> Result<Map, EvmError>
    where
        Key: Serialize,
    {
        Ok(Map::new(self.key(key)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_tests::FakeExt;

    #[test]
    fn test_scalar_bytes() {
        let mut ext = FakeExt::new();
        let scalar = Scalar::new(H256::from(0));

        // 1) length=30
        let expected = format!("012345678901234567890123456789");
        assert!(scalar.set_bytes(&mut ext, &expected).is_ok());
        let value = scalar.get_bytes::<String>(&ext);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());

        // 2) length=31
        let expected = format!("0123456789012345678901234567890");
        assert!(scalar.set_bytes(&mut ext, &expected).is_ok());
        let value = scalar.get_bytes::<String>(&ext);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());

        // 3) length=32
        let expected = format!("01234567890123456789012345678901");
        assert!(scalar.set_bytes(&mut ext, &expected).is_ok());
        let value = scalar.get_bytes::<String>(&ext);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());

        // 4) length=43
        let expected = format!("012345678901234567890123456789012");
        assert!(scalar.set_bytes(&mut ext, &expected).is_ok());
        let value = scalar.get_bytes::<String>(&ext);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());
    }

    #[test]
    fn test_scalar_u256() {
        let mut ext = FakeExt::new();
        let scalar = Scalar::new(H256::from(0));

        let expected = U256::from(0x123456);
        assert!(scalar.set(&mut ext, expected.clone()).is_ok());
        let value = scalar.get(&ext);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), expected.clone());
    }

    #[test]
    fn test_array_simple() {
        let mut ext = FakeExt::new();
        let length = 7u64;
        let array = Array {
            position: H256::from(0),
        };
        // 1) length
        assert!(array.set_len(&mut ext, length).is_ok());
        assert_eq!(array.get_len(&ext).unwrap(), length);

        // 2) array[1] = 0x1234
        let index = 1;
        let expected = U256::from(0x1234);
        assert!(array.set(&mut ext, index, &expected).is_ok());
        let value = array.get(&ext, index);
        assert_eq!(value.unwrap(), expected.clone());

        // 3) array[3] = 0x2234
        let index = 3;
        let expected = U256::from(0x2234);
        assert!(array.set(&mut ext, index, &expected).is_ok());
        let value = array.get(&ext, index);
        assert_eq!(value.unwrap(), expected.clone());
    }

    #[test]
    fn test_array_with_sub_array() {
        let mut ext = FakeExt::new();
        let mut array = Array::new(H256::from(0));

        // 1) length = 7
        let length = 7;
        assert!(array.set_len(&mut ext, length).is_ok());
        assert_eq!(array.get_len(&ext).unwrap(), length);

        // 2) array[1].len = 8
        let index = 1;
        let subarray_length = 8;
        let subarray = array.get_array(index);
        assert!(subarray.set_len(&mut ext, subarray_length).is_ok());
        assert_eq!(subarray.get_len(&mut ext).unwrap(), subarray_length);

        // 3) array[1][2] = 0x1234
        let index = 2;
        let expected = U256::from(0x1234);
        assert!(subarray.set(&mut ext, index, &expected).is_ok());
        assert_eq!(subarray.get(&ext, index).unwrap(), expected);

        // 4) array[1][4] = 0x2234
        let index = 4;
        let expected = U256::from(0x2234);
        assert!(subarray.set(&mut ext, index, &expected).is_ok());
        assert_eq!(subarray.get(&ext, index).unwrap(), expected);
    }

    #[test]
    fn test_array_with_sub_map() {
        let mut ext = FakeExt::new();
        let mut array = Array::new(H256::from(0));

        // 1) length = 7
        let length = 7;
        assert!(array.set_len(&mut ext, length).is_ok());
        assert_eq!(array.get_len(&ext).unwrap(), length);

        // 2) array[1][2] = 0x1234
        let index = 1;
        let key = U256::from(2);
        let submap = array.get_map(index);
        let expected = U256::from(0x1234);
        assert!(submap.set(&mut ext, &key, expected).is_ok());
        assert_eq!(submap.get::<U256>(&ext, &key).unwrap(), expected);

        // 4) array[1]["key"] = "1234"
        let key = String::from("key");
        let expected = String::from("1234");
        assert!(submap
            .set_bytes::<String, String>(&mut ext, &key, &expected)
            .is_ok());
        assert_eq!(
            submap.get_bytes::<String, String>(&ext, &key).unwrap(),
            expected.clone()
        );
    }

    #[test]
    fn test_map_simple() {
        let mut ext = FakeExt::new();
        let map = Map::new(H256::from(1));

        // 1) map["key"] = "value"
        let key = U256::from(1);
        let value = U256::from(0x1234);
        assert!(map.set(&mut ext, &key, value).is_ok());
        assert_eq!(map.get(&ext, &key).unwrap(), value);

        // 2) map[0] = "1234567890"
        let key = U256::from(1);
        let value = String::from("1234567890");
        assert!(map.set_bytes(&mut ext, &key, &value).is_ok());
        assert_eq!(
            map.get_bytes::<U256, String>(&ext, &key).unwrap(),
            value.clone()
        );

        // 3) map[0] = "123456789012345678901234567890123"
        let key = U256::from(1);
        let value = String::from("123456789012345678901234567890123");
        assert!(map.set_bytes(&mut ext, &key, &value).is_ok());
        assert_eq!(map.get_bytes::<U256, String>(&ext, &key).unwrap(), value);

        // 4) map["key"] = 0x1234;
        let key = String::from("key");
        let value = U256::from(0x1234);
        assert!(map.set(&mut ext, &key, value).is_ok());
        assert_eq!(map.get(&ext, &key).unwrap(), value);;
    }

    #[test]
    fn test_map_with_sub_array() {
        let mut ext = FakeExt::new();
        let mut map = Map::new(H256::from(1));

        // 1) map["key1"]["key2"] = "1234567890"
        let key1 = String::from("key1");
        let index = 2u64;
        let value = String::from("1234567890");
        let sub_array = map.get_array(&key1).unwrap();
        assert!(sub_array.set_bytes(&mut ext, index.clone(), &value).is_ok());
        assert_eq!(
            *sub_array.get_bytes::<String>(&ext, index.clone()).unwrap(),
            value.clone()
        );

        // 2) map["key1"][2] = "1234567890"
        let key1 = String::from("key1");
        let index = 4u64;
        let value = String::from("1234567890");
        let sub_array = map.get_array(&key1).unwrap();
        assert!(sub_array.set_bytes(&mut ext, index.clone(), &value).is_ok());
        assert_eq!(
            *sub_array.get_bytes::<String>(&ext, index.clone()).unwrap(),
            value.clone()
        );
    }

    #[test]
    fn test_map_with_sub_map() {
        let mut ext = FakeExt::new();
        let mut map = Map::new(H256::from(1));

        // 1) map["key1"]["key2"] = "1234567890"
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let value = String::from("1234567890");
        let sub_map = map.get_map(&key1).unwrap();
        assert!(sub_map.set_bytes(&mut ext, &key2, &value).is_ok());
        assert_eq!(
            sub_map.get_bytes::<String, String>(&ext, &key2).unwrap(),
            value.clone()
        );

        // 2) map["key1"][2] = "1234567890"
        let key1 = String::from("key1");
        let key2 = U256::from(2);
        let value = String::from("1234567890");
        let sub_map = map.get_map(&key1).unwrap();
        assert!(sub_map.set_bytes(&mut ext, &key2, &value).is_ok());
        assert_eq!(
            sub_map.get_bytes::<_, String>(&ext, &key2).unwrap(),
            value.clone()
        );
    }
}
