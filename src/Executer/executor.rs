use crate::Loader::walker::{walk, walk_str, walk_with_delimiter, walk_with_size};
use std::{convert::TryFrom, io::BufReader};
use crate::instruction::{OpCode};
use super::stack::{Stack, Stackish};
use super::value::{I32, I64, F32, F64, ValueType};


trait Serializable{
  fn serialize(&self)->Result<Vec<u8>, SerializeError>;

}

impl Serializable for I32{
  fn serialize(&self)->Result<Vec<u8>, SerializeError> {
    let serial = self.inner.to_ne_bytes();
    Ok(serial.to_vec())
  }
}



impl TryFrom<u8> for ValueType {
    type Error = ();

    fn try_from(val: u8) -> Result<ValueType, ()> {
        match val {
            0 => Ok(ValueType::I32),
            1 => Ok(ValueType::I64),
            2 => Ok(ValueType::F32),
            3 => Ok(ValueType::F64),
            4 => Ok(ValueType::V128),
            5 => Ok(ValueType::RefFunc),
            6 => Ok(ValueType::RefNull),
            7 => Ok(ValueType::RefExtern),
            _ => Err(()),
        }
    }
}

// serialize
impl From<I32> for Vec<u8>{
  fn from(i: I32) -> Self {
    let serial = i.inner.to_ne_bytes();
    serial.to_vec()
  }
}
// deserialize
impl TryFrom<Vec<u8>> for I32{
  type Error = ();
  fn try_from(v: Vec<u8>) -> Result<Self, ()>{
    if v.len() != 4{
      return Err(());
    }
    let mut buffer = [0;4];
    for i in 0..4{
      buffer[i] = v[i];
    }
    let data = i32::from_le_bytes(buffer);
    let mut result = I32{inner: data};
    Ok(result)
  }
}

#[derive(Debug)]
enum SerializeError{
  TypeNotFound,
}




struct Store{
}


trait ExecuterTrait{
  fn handleOp(self: &mut Self, inst: Instruction);
}
pub struct Executer{
  pc: u32,  
  stack: Stack,
  store: Store,
}

pub struct Instruction{
  opCode: OpCode,
  params: Vec<u8>
}





impl ExecuterTrait for Executer{
  fn handleOp(self: &mut Self, inst: Instruction){
    match inst.opCode{
      OpCode::I32Const =>{
        // let result:Result<I32,_> = inst.params.try_into();
        // TODO: error handling
        self.stack.push(ValueType::I32, inst.params);
      }
      OpCode::End => {
  
      }
    }
  }
}
