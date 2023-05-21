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

