use std::io::{Error, ErrorKind};
use bincode::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, PartialEq)]
pub enum Data {
    INumber(i64),
    UNumber(u64),
    UTF8(String),
    ByteArray(Vec<u8>),
    None
}

impl TryInto<i64> for Data {
    type Error = Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        if let Data::INumber(data) = self {
            return Ok(data)
        }
        Err(Error::new(ErrorKind::NotFound, "Bad data"))
    }
}

impl TryInto<u64> for Data {
    type Error = Error;

    fn try_into(self) -> Result<u64, Self::Error> {
        if let Data::UNumber(data) = self {
            return Ok(data)
        }
        Err(Error::new(ErrorKind::NotFound, "Bad data"))
    }
}

impl TryInto<String> for Data {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        if let Data::UTF8(data) = self {
            return Ok(data)
        }
        Err(Error::new(ErrorKind::NotFound, "Bad data"))
    }
}

impl TryInto<Vec<u8>> for Data {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if let Data::ByteArray(data) = self {
            return Ok(data)
        }
        Err(Error::new(ErrorKind::NotFound, "Bad data"))
    }
}