use std::fmt;

use super::executor::Param;
use super::utils::{get_type_size, read_one};
use super::value::{I32, ValueType};
use super::value::*;
pub trait Stackish{
  type Element;
  fn pop(&mut self)->Result<Self::Element, StackErr>;
  fn push(&mut self, value: Self::Element);
    // This is not safe: val_type and T are not guaranteed to be consistent
  fn size(&self)->usize;
  fn empty(&self)->bool;
}


pub struct Label{
  pub arity: u8,
  pub target: (u32, u32) // (start pc, end pc)
}

pub struct ActivationFrame{
  pub index: u8, // index of this activation frame in bookkeeping
  pub locals: Vec<Box<Param>>,
  // TODO: reference to its own module instance  
}

pub struct AfMeta{
  pub len: u8, // lenght of the data
  pub reference: Vec<u8> // the index of each arguments in the corresponding ActivationFrame data
}

#[derive(Default)]
pub struct Stack{
  inner: Vec<StackElement>,
}
#[derive(Debug)]
pub enum StackErr{
  ErrorPop
}

pub enum StackElement{
  I32(Box<I32>),
  I64(Box<I64>),
  F32(Box<F32>),
  F64(Box<F64>),
  V128(Box<V128>),
  RefNull(Box<RefNull>),
  RefFunc(Box<RefFunc>),
  RefExtern(Box<RefExtern>),
  Label(Box<Label>),
  Activation(Box<ActivationFrame>),
}

impl fmt::Debug for StackElement {
  fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
      match self{
        StackElement::I32(v)=>{
          println!("{}", (*v).inner);
        }
        _ => println!("Debug trait not implemented for this stack element")
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
      let stack_element = result.unwrap();
      Ok(stack_element)
  }
  fn push(&mut self, value: StackElement){
      self.inner.push(value);
  }
  fn size(&self)->usize {
    return self.inner.len() as usize;
  }
  fn empty(&self)->bool {
    return self.size() == 0;
  }
}