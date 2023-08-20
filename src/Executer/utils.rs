use super::value::ValueType;
use crate::Loader::walker::{Segable, Walkable};
use std::io::{BufReader, Read};
pub fn get_type_size(value_type: &ValueType) -> usize{
  match value_type{
    ValueType::I32=>{
      // let result = i32::seg(&mut buf_reader);
      // let size = result.unwrap().len();
      // return size;
      return 4;
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
    ValueType::Label=>{
      return 8;
    }
    ValueType::Activation=>{
      return 8;
    }
  }
}



pub fn read_one(data: &Vec<u8>, value_type: &ValueType)->Vec<u8>{
  let data_size = get_type_size(value_type);
  let length = data.len();
  let ret = data.as_slice()[length - data_size..].to_vec();
  return ret;
}

pub fn LEB_to_native(leb: &Vec<u8>, value_type: &ValueType)->Result<Vec<u8>, ()>{
  let mut buf_reader: BufReader<_> = BufReader::new(leb.as_slice());
  let ret: Vec<u8>;
  match value_type{
    ValueType::I32=>{
      let mut result = i32::walk(&mut buf_reader);
      if result.is_ok(){
        let result = i32::to_le_bytes(result.unwrap());
        ret = result.to_vec();
        return Ok(ret);
      } else{
        return Err(());
      }
    }
    ValueType::I64=>{
    }
    ValueType::F32=>{

    }
    ValueType::F64=>{

    }
    _=>{
      return Err(());
    }
  }
  return Err(());
}