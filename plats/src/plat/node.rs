use bitcode::{Decode, Encode};
use serde::Serialize;

#[derive(Clone, Debug, Encode, Decode)]
pub struct Node {
    pub flag: i32,
    pub children: [u32; 8],
}
