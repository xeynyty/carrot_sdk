use crate::utils::Work;
use bincode::{Decode, Encode};
use crate::data::Data;

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
pub struct Request {
    work: Work,
    key: Option<u32>,
    data: Data,
    iat: u64
}

impl Request {
    pub fn new() -> Self {
        Self {
            work: Work::None,
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

    pub fn write(self) -> Self {
        Self {
            work: Work::Write, ..self
        }
    }
    pub fn read(self) -> Self {
        Self {
            work: Work::Read, ..self
        }
    }
    pub fn remove(self) -> Self {
        Self {
            work: Work::Remove, ..self
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
impl SetData<i64> for Request {
    fn set_data(self, data: i64) -> Self {
        Self {
            data: Data::INumber(data),
            ..self
        }
    }
}
impl SetData<i32> for Request {
    fn set_data(self, data: i32) -> Self {
        Self {
            data: Data::INumber(data as i64),
            ..self
        }
    }
}

// Unsigned integer
impl SetData<u64> for Request {
    fn set_data(self, data: u64) -> Self {
        Self {
            data: Data::UNumber(data),
            ..self
        }
    }
}
impl SetData<u32> for Request {
    fn set_data(self, data: u32) -> Self {
        Self {
            data: Data::UNumber(data as u64),
            ..self
        }
    }
}

// String and &str
impl SetData<String> for Request {
    fn set_data(self, data: String) -> Self {
        Self {
            data: Data::UTF8(data),
            ..self
        }
    }
}
impl SetData<&str> for Request {
    fn set_data(self, data: &str) -> Self {
        Self {
            data: Data::UTF8(data.into()),
            ..self
        }
    }
}

// Vec<u8>
impl SetData<Vec<u8>> for Request {
    fn set_data(self, data: Vec<u8>) -> Self {
        Self {
            data: Data::ByteArray(data),
            ..self
        }
    }
}

impl Default for Request {
    fn default() -> Self {
        Request::new()
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = bincode::error::DecodeError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (res, _len): (Request, usize) = bincode::decode_from_slice(&value, bincode::config::standard())?;
        Ok(res)
    }
}
impl TryInto<Vec<u8>> for Request {
    type Error = bincode::error::EncodeError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::encode_to_vec(self, bincode::config::standard())
    }
}
impl TryFrom<&[u8]> for Request {
    type Error = bincode::error::DecodeError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (res, _len): (Request, usize) = bincode::decode_from_slice(value, bincode::config::standard())?;
        Ok(res)
    }
}
