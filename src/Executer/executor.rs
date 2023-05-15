
enum Number{
  I32,
  I64,
  F32,
  F64
}


enum Vector{
  V128,
}

enum Ref{
  RefNull,
  RefFunc,
  RefExtern
}
enum ValueType{
  Number(Number),
  Vector(Vector),
  Ref(Ref)
}

enum SerializeError{
  TypeNotFound,
}



fn serialize<T>(value: T, val_type: ValueType) -> Result<Vec<u8>, SerializeError>{
  match val_type {
    ValueType::Number(number_type)=>{
      match number_type{
        Number::I32=>{
          
        }
        Number::I64=>{

        }
        Number::F32=>{

        }
        Number::F64=>{

        }
      }
    }
    ValueType::Vector(vector_type)=>{
    }
    ValueType::Ref(ref_type)=>{
      match ref_type{
        Ref::RefNull=>{

        }
        Ref::RefExtern=>{
        }
        Ref::RefFunc=>{
        }
      }
    }
  }
  return Err(SerializeError::TypeNotFound);
}



trait Stackish{
  fn pop(&self){

  }

  fn push(&mut self, val_type: ValueType, value: Vec<u8>){
    
  }
}
struct Stack{
  inner: Vec<u8>
}

pub struct Executer{

}