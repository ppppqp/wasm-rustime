use crate::Loader::walker::{Walkable};
use std::io::{BufReader};
use crate::Executer::stack::{ActivationFrame, Label};
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
  RefExtern = 7,
  Label = 8,
  Activation = 9,
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

impl TryFrom<ActivationFrame> for Vec<u8>{
  type Error = ();
  fn try_from(af: ActivationFrame) -> Result<Vec<u8>, ()>{
    let mut ret = vec![];
    ret.push(af.index);
    ret.append(&mut af.locals.to_vec());
    return Ok(ret);
  }
}

impl TryFrom<Vec<u8>> for ActivationFrame{
  type Error = ();
  fn try_from(v: Vec<u8>) -> Result<ActivationFrame, ()>{
    let i = 0;
    if v.len() == 0 {
      return Err(());
    }
    let af = ActivationFrame{
      index: v[0],
      locals: v[1..].to_vec()
    };
    Ok(af)
  }
}

impl TryFrom<Label> for Vec<u8>{
  type Error = ();
  fn try_from(l: Label) -> Result<Vec<u8>, ()>{
    let mut ret = vec![];
    ret.push(l.arity);
    ret.append(&mut u32::to_le_bytes(l.target.0).to_vec());
    ret.append(&mut u32::to_le_bytes(l.target.1).to_vec());
    Ok(ret)
  }
}

impl TryFrom<Vec<u8>> for Label{
  type Error = ();
  fn try_from(v: Vec<u8>) -> Result<Label, ()>{
    let buffer = [v[1], v[2], v[3], v[4]];
    let target1 = u32::from_le_bytes(buffer);
    let buffer =  [v[5], v[6], v[7], v[8]];
    let target2 = u32::from_le_bytes(buffer);

    let ret = Label{
      arity: v[0],
      target: (target1, target2)
    };
    Ok(ret)
  }
}