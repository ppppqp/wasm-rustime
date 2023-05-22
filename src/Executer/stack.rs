use std::fmt;

use super::utils::get_type_size;
use super::value::{I32, ValueType};

pub trait Stackish{
  type Element;
  fn pop(&mut self)->Result<Self::Element, StackErr>;
  fn push(&mut self, val_type: ValueType, value: &Vec<u8>);
    // This is not safe: val_type and T are not guaranteed to be consistent
  fn size(&self)->usize;
  fn empty(&self)->bool;
}
#[derive(Default)]
pub struct Stack{
  inner: Vec<u8>
}
#[derive(Debug)]
pub enum StackErr{
  ErrorPop
}
pub struct StackElement{
  element_type: ValueType,
  data: Vec<u8>,
}

impl fmt::Debug for StackElement {
  fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
      match self.element_type{
        ValueType::I32=>{
          let buffer = self.data.to_vec();
          let result:Result<I32, ()> = buffer.try_into();
          if result.is_ok(){
            println!("{}", result.unwrap().inner);
          }
        }
        ValueType::I64=>{
        }
        ValueType::F32=>{

        }
        ValueType::F64=>{

        }
        ValueType::V128=>{

        }
        ValueType::RefNull=>{

        }
        ValueType::RefFunc=>{

        }
        ValueType::RefExtern=>{

        }

      }
      Ok(())
  }
}
impl Stackish for Stack{
  type Element = StackElement;
  fn pop(&mut self)->Result<Self::Element, StackErr>{
      let result = self.inner.pop();
      if result.is_none(){
        // TODO: error
        return Err(StackErr::ErrorPop);
      }
      let result = result.unwrap().try_into();
      if result.is_err(){
        // TODO: error
        return Err(StackErr::ErrorPop);
      }
      let value_type:ValueType = result.unwrap();
      // get an element from the stack
      let data_size = get_type_size(&value_type, &self.inner);
      let length = self.inner.len();
      let data = self.inner.as_slice()[length - data_size..].to_vec();
      for _ in 0..data_size{
        self.inner.pop();
      }
      let stack_element = StackElement{element_type: value_type, data };
      Ok(stack_element)
  }
  fn push(&mut self, val_type: ValueType, value: &Vec<u8>){
      let new_vec = value.to_vec();
      let result:Result<Vec<u8>, _> = new_vec.try_into();
      if result.is_err(){
        //TODO
        return;
      }
      self.inner.append(&mut result.unwrap());
      self.inner.push(val_type as u8);
  }
  fn size(&self)->usize {
    return self.inner.len() as usize;
  }
  fn empty(&self)->bool {
    return self.size() == 0;
  }
}