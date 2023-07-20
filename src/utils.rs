use bincode::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, PartialEq)]
pub enum Work {
    Read, Write, Remove
}
