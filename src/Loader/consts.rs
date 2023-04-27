// for binary

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