use super::value::ValueType;
use crate::Loader::walker::{Segable};
use std::io::{BufReader};
pub fn get_type_size(value_type: &ValueType, data: &Vec<u8>) -> usize{
  let mut buf_reader = BufReader::new(data.as_slice());
  match value_type{
    ValueType::I32=>{
      let result = i32::seg(&mut buf_reader);
      let size = result.unwrap().len();
      return size;
    }
    ValueType::I64=>{
      return 8;
    }
    ValueType::F32=>{
      return 4;
    }
    ValueType::F64=>{
      return 8;
    }
    ValueType::V128=>{
      return 16;
    }
    ValueType::RefNull=>{
      return 8;
    }
    ValueType::RefFunc=>{
      return 8;
    }
    ValueType::RefExtern=>{
      return 8;
    }
    // TODO: Reference types are opaque????
  }
}