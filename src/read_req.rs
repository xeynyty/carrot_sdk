use bincode::{Decode, Encode};
use crate::utils::Work;

#[derive(Clone, Debug, Encode, Decode, PartialEq)]
pub struct ReadReq {
    work: Work,
    key: Option<u32>,
}

impl ReadReq {
    pub fn new() -> Self {
        Self {
            work: Work::Read,
            key: None
        }
    }
    pub fn set_key(self, key: Option<u32>) -> Self {
        Self {
            key, ..self
        }
    }

    pub fn key(&self) -> Option<u32> {
        self.key
    }
    pub fn work(&self) -> Work {
        self.work
    }
}

impl Default for ReadReq {
    fn default() -> Self {
        ReadReq::new()
    }
}

impl TryFrom<Vec<u8>> for ReadReq {
    type Error = bincode::error::DecodeError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (res, _len): (ReadReq, usize) = bincode::decode_from_slice(&value, bincode::config::standard())?;
        Ok(res)
    }
}
impl TryInto<Vec<u8>> for ReadReq {
    type Error = bincode::error::EncodeError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::encode_to_vec(self, bincode::config::standard())
    }
}
