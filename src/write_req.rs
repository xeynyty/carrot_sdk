use crate::utils::Work;
use bincode;
use bincode::{Decode, Encode};
use crate::data::Data;

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
pub struct WriteReq {
    work: Work,
    key: Option<u32>,
    data: Data,
    iat: u64
}

impl WriteReq {
    pub fn new() -> Self {
        Self {
            work: Work::Write,
            key: None,
            data: Data::UNumber(0),
            iat: 0,
        }
    }
    pub fn set_key(self, key: Option<u32>) -> Self {
        Self {
            key, ..self
        }
    }
    pub fn set_iat(self, iat: u64) -> Self {
        Self {
            iat, ..self
        }
    }

    pub fn work(&self) -> Work {
        self.work
    }
    pub fn key(&self) -> Option<u32> {
        self.key
    }
    pub fn data(&self) -> Data {
        self.data.clone()
    }
    pub fn iat(&self) -> u64 {
        self.iat
    }

}


/// Set input data to Data enum
pub trait SetData<T> {
    fn set_data(self, data: T) -> Self;
}

// Integer
impl SetData<i64> for WriteReq {
    fn set_data(self, data: i64) -> Self {
        Self {
            data: Data::INumber(data),
            ..self
        }
    }
}
impl SetData<i32> for WriteReq {
    fn set_data(self, data: i32) -> Self {
        Self {
            data: Data::INumber(data as i64),
            ..self
        }
    }
}

// Unsigned integer
impl SetData<u64> for WriteReq {
    fn set_data(self, data: u64) -> Self {
        Self {
            data: Data::UNumber(data),
            ..self
        }
    }
}
impl SetData<u32> for WriteReq {
    fn set_data(self, data: u32) -> Self {
        Self {
            data: Data::UNumber(data as u64),
            ..self
        }
    }
}

// String and &str
impl SetData<String> for WriteReq {
    fn set_data(self, data: String) -> Self {
        Self {
            data: Data::UTF8(data),
            ..self
        }
    }
}
impl SetData<&str> for WriteReq {
    fn set_data(self, data: &str) -> Self {
        Self {
            data: Data::UTF8(data.into()),
            ..self
        }
    }
}

impl Default for WriteReq {
    fn default() -> Self {
        WriteReq::new()
    }
}

impl TryFrom<Vec<u8>> for WriteReq {
    type Error = bincode::error::DecodeError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (res, _len): (WriteReq, usize) = bincode::decode_from_slice(&value, bincode::config::standard())?;
        Ok(res)
    }
}
impl TryInto<Vec<u8>> for WriteReq {
    type Error = bincode::error::EncodeError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::encode_to_vec(self, bincode::config::standard())
    }
}
