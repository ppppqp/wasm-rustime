use self::OpCode::*;
use std::slice::Iter;
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode{
  Unreachable = 0x00,
  Nop = 0x01,
  Block = 0x02,
  Loop = 0x03,
  Call = 0x10,
  BrIf = 0x0D,
  LocalGet = 0x20,
  LocalSet = 0x21,
  LocalTee = 0x22,
  GlobalGet = 0x23,
  GlobalSet = 0x24,
  BlockTypeE = 0x40,
  I32Const = 0x41,
  I32Eq = 0x46,
  I32Ne = 0x47,
  I32Add = 0x6A,
  I32Or = 0x72,
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
      0x03 => Ok(OpCode::Loop),
      0x10 => Ok(OpCode::Call),
      0x0D => Ok(OpCode::BrIf),
      0x20 => Ok(OpCode::LocalGet),
      0x21 => Ok(OpCode::LocalSet),
      0x22 => Ok(OpCode::LocalTee),
      0x23 => Ok(OpCode::GlobalGet),
      0x24 => Ok(OpCode::GlobalSet),
      0x40 => Ok(OpCode::BlockTypeE),
      0x46 => Ok(OpCode::I32Eq),
      0x47 => Ok(OpCode::I32Ne),
      0x6A => Ok(OpCode::I32Add),
      0x72 => Ok(OpCode::I32Or),
      0xb => Ok(OpCode::End),
      _ => Err(()),
    }
  }
}
