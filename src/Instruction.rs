use self::OpCode::*;
use std::slice::Iter;
pub enum OpCode{
  I32Const = 0x41,
  End = 0xb
}

impl TryFrom<u8> for OpCode {
  type Error = ();
  fn try_from(v: u8) -> Result<Self, ()> {
    match v{
      0x41 => Ok(OpCode::I32Const),
      0xb => Ok(OpCode::End),
      _ => Err(()),
    }
  }
}
