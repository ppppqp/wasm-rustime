use super::value::ValueType;

pub fn get_type_size(value_type: &ValueType) -> usize{
  match value_type{
    ValueType::I32=>{
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
  }
}