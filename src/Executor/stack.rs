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

#[derive(Clone)]
pub struct Label{
  pub arity: u8,
  pub target: (u32, u32) // (start pc, end pc)
}

#[derive(Clone)]
pub struct ActivationFrame{
  pub index: u8, // index of this activation frame in bookkeeping
  pub locals: Vec<Box<Param>>,
  // TODO: reference to its own module instance  
}

pub struct AfMeta{
  pub len: u8, // length of the data
  pub position: usize, // the position of the activation frame in stack 
}

#[derive(Default)]
pub struct Stack{
  inner: Vec<StackElement>,
}
#[derive(Debug)]
pub enum StackErr{
  ErrorPop,
  ErrorGet
}

#[derive(Clone)]
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
        StackElement::I64(v)=>{
          println!("{}", (*v).inner);
        }
        StackElement::F32(v)=>{
          println!("{}", (*v).inner);
        }
        StackElement::F64(v)=>{
          println!("{}", (*v).inner);
        }
        StackElement::RefNull(_)=>{
          println!("RefNull");
        }
        StackElement::RefFunc(_)=>{
          println!("RefFunc");
        }
        StackElement::RefExtern(_)=>{
          println!("RefExtern");
        }
        StackElement::Label(_)=>{
          println!("RefLabel");
        }
        StackElement::Activation(_)=>{
          println!("Activation");
        }
        _ => println!("Debug trait not implemented for this stack element")
      }
      Ok(())
  }
}

impl Stack{
  pub fn get(&mut self, position: usize)->Result<&mut StackElement, StackErr>{
    // get the element at the position
    if position < self.inner.len(){
      return Ok(&mut self.inner[position]);
    } else {
      return Err(StackErr::ErrorGet);
    }
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