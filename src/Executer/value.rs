use crate::Loader::walker::{Walkable};
use std::io::{BufReader};
pub struct I32{
  pub inner: i32
}
pub struct I64{
  pub inner: i64
}
pub struct F32{
  pub inner: f32
}
pub struct F64{
  pub inner: f64
}

#[derive(Debug)]
pub enum ValueType{
  I32 = 0,
  I64 = 1,
  F32 = 2,
  F64 = 3,
  V128 = 4,
  RefNull = 5,
  RefFunc = 6,
  RefExtern = 7
}

impl From<I32> for Vec<u8>{
  fn from(i: I32) -> Self {
    let mut encoded = Vec::new();
    let mut num = i.inner;
    loop {
        let byte: u8 = (num as u8) & 0x7F;
        num >>= 7;
        if num == 0 {
            encoded.push(byte);
            break;
        } else {
            encoded.push(byte | 0x80);
        }
    }
    encoded
  }
}

impl TryFrom<Vec<u8>> for I32{
  type Error = ();
  fn try_from(v: Vec<u8>) -> Result<Self, ()>{
    let mut buf_reader = BufReader::new(v.as_slice());
    let result = i32::walk(&mut buf_reader);
    match result{
      Ok(i) => Ok(I32 { inner:  i}),
      Err(_) => Err(()),
    }
  }
}
