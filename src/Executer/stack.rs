use super::utils::get_type_size;
use super::value::ValueType;

pub trait Stackish{
  type Element;
  fn pop(&mut self)->Result<Self::Element, StackErr>;
  fn push(&mut self, val_type: ValueType, value: Vec<u8>);
  // This is not safe: val_type and T are not guaranteed to be consistent
  fn size(&self)->usize;
  fn empty(&self)->bool;
}
pub struct Stack{
  inner: Vec<u8>
}
pub enum StackErr{
  ErrorPop
}
pub struct StackElement{
  elementType: ValueType,
  data: Vec<u8>,
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
      let data_size = get_type_size(&value_type);
      let length = self.inner.len();
      let data = self.inner.as_slice()[length - data_size..].to_vec();
      for _ in 0..data_size{
        self.inner.pop();
      }
      let stack_element = StackElement{elementType: value_type, data };
      Ok(stack_element)
  }
  fn push(&mut self, val_type: ValueType, value: Vec<u8>){
      let result:Result<Vec<u8>, _> = value.try_into();
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