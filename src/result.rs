use bincode::{Decode, Encode};
use crate::data::Data;

#[derive(Clone, Debug, Encode, Decode)]
pub struct Response {
    key: u32,
    data: Data,
}

impl Response {
    pub fn new(key: u32, data: Data) -> Self {
        Self {
            key, data
        }
    }
    pub fn key(&self) -> u32 {
        self.key
    }
    pub fn data(&self) -> Data {
        self.data.clone()
    }
}

impl TryFrom<Vec<u8>> for Response {
    type Error = bincode::error::DecodeError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (res, _len): (Response, usize) = bincode::decode_from_slice(&value, bincode::config::standard())?;
        Ok(res)
    }
}
impl TryFrom<&[u8]> for Response {
    type Error = bincode::error::DecodeError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (res, _len): (Response, usize) = bincode::decode_from_slice(value, bincode::config::standard())?;
        Ok(res)
    }
}

impl TryInto<Vec<u8>> for Response {
    type Error = bincode::error::EncodeError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::encode_to_vec(self, bincode::config::standard())
    }
}