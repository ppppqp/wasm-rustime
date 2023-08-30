// for binary

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Type{
  Func = 0x60,
  I32 = 0x7F,
  I64 = 0x7E,
  F32 = 0x7D,
  F64 = 0x7C,
  Vec = 0x7B,
  FuncRef = 0x70,
  ExternRef = 0x6F,
  LimitsMin = 0x00,
  LimitsMinMax = 0x01,
}

impl TryFrom<u8> for Type{
  type Error = ();
  fn try_from(value: u8) -> Result<Self, Self::Error> {
      match(value){
        0x60 => Ok(Type::Func),
        0x7F => Ok(Type::I32),
        0x7E => Ok(Type::I64),
        0x7D => Ok(Type::F32),
        0x7C => Ok(Type::F64),
        0x7B => Ok(Type::Vec),
        0x70 => Ok(Type::FuncRef),
        0x6F => Ok(Type::ExternRef),
        0x00 => Ok(Type::LimitsMin),
        0x01 => Ok(Type::LimitsMinMax),
        _ => Err(()),
      }
  }
}

pub enum GlobalType{
  Const = 0x00,
  Var = 0x01,
}

pub enum ExportType{
  Func = 0x00,
  Tab = 0x01,
  Mem = 0x02,
  Global = 0x03, 
}

pub enum LimitFlag{
  PASSIVE = 0x00,
  ACTIVE = 0x01,
}