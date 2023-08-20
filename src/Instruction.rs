use self::OpCode::*;
use std::slice::Iter;
#[derive(Debug)]
pub enum OpCode{
  Unreachable = 0x00,
  Nop = 0x01,
  Block = 0x02,
  Call = 0x10,
  LocalGet = 0x20,
  LocalSet = 0x21,
  LocalTee = 0x22,
  GlobalGet = 0x23,
  GlobalSet = 0x24,
  BlockTypeE = 0x40,
  I32Const = 0x41,

  End = 0xb
}

impl TryFrom<u8> for OpCode {
  type Error = ();
  fn try_from(v: u8) -> Result<Self, ()> {
    match v{
      0x41 => Ok(OpCode::I32Const),
      0x00 => Ok(OpCode::Unreachable),
      0x01 => Ok(OpCode::Nop),
      0x02 => Ok(OpCode::Block),
      0x10 => Ok(OpCode::Call),
      0x20 => Ok(OpCode::LocalGet),
      0x21 => Ok(OpCode::LocalSet),
      0x22 => Ok(OpCode::LocalTee),
      0x23 => Ok(OpCode::GlobalGet),
      0x24 => Ok(OpCode::GlobalSet),
      0xb => Ok(OpCode::End),
      _ => Err(()),
    }
  }
}
