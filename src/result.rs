use crate::data::Data;

pub struct Response {
    key: u32,
    data: Option<Data>,
}

impl Response {
    pub fn new(key: u32, data: Option<Data>) -> Self {
        Self {
            key, data
        }
    }
    pub fn key(&self) -> u32 {
        self.key
    }
    pub fn data(&self) -> Option<Data> {
        self.data.clone()
    }
}