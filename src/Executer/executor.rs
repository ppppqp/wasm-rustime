use crate::Loader::walker::{Walkable, Segable, walk, walk_str, walk_with_delimiter, walk_with_size};
use std::{convert::TryFrom, io::BufReader, io::Read};
use crate::instruction::{OpCode};
use super::stack::*;
use super::value::{I32, I64, F32, F64, ValueType};
use super::value::*;
use crate::Module::module::{Module, Function, Code};
use super::utils::{LEB_to_native, get_type_size};


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




#[derive(Default)]
struct Store{
}


pub trait ExecuterTrait{
  fn handle_inst(self: &mut Self, inst: &Instruction)->Result<ExecutionRes, ExecutionErr>;
  fn run(self: &mut Self, program: &Vec<Instruction>);
  fn run_function(self: &mut Self, module: &Module, index: usize);
  }
#[derive(Default)]
pub struct Executer{
  pc: usize,  
  stack: Stack,
  store: Store,
  af_meta: Vec<AfMeta>
}

pub struct Instruction{
  op_code: OpCode,
  params: Vec<Param>
}

pub enum Param{
  I32(Box<I32>),
  I64(Box<I64>),
  F32(Box<F32>),
  F64(Box<F64>),
  V128(Box<V128>),
}

pub struct ExecutionRes{
  // data: Vec<u8>,
  // value_types: Vec<ValueType>
}
pub enum ExecutionErr{
  Trap,
  Terminate
}


impl From<StackElement> for Param{
  fn from(value: StackElement) -> Self {
      match value {
          StackElement::I32(v)  => Param::I32(v),
          StackElement::I64(v)  => Param::I64(v),
          StackElement::F32(v)  => Param::F32(v),
          StackElement::F64(v)  => Param::F64(v),
          _ => panic!("type converstion error"),
      }
  }
}

impl From<Param> for StackElement{
  fn from(value: Param) -> Self {
      match value {
          Param::I32(v)  => StackElement::I32(v),
          Param::I64(v)  => StackElement::I64(v),
          Param::F32(v)  => StackElement::F32(v),
          Param::F64(v)  => StackElement::F64(v),
          Param::V128(v)  => StackElement::V128(v),
          _ => panic!("type converstion error"),
      }
  }
}

impl Clone for Param{
  fn clone(&self) -> Self {
      match self{
        Param::I32(v)  => Param::I32(Box::new(I32{inner: (*v).inner})),
        Param::I64(v)  => Param::I64(Box::new(I64{inner: (*v).inner})),
        Param::F32(v)  => Param::F32(Box::new(F32{inner: (*v).inner})),
        Param::F64(v)  => Param::F64(Box::new(F64{inner: (*v).inner})),
        Param::V128(v)  => Param::V128(Box::new(V128{inner: (*v).inner.to_vec()}))
      }
  }
}

impl From<NativeNumeric> for Param{
  fn from(value: NativeNumeric) -> Self {
    match value{
      NativeNumeric::I32(v) => Param::I32(Box::new(I32{inner: v})),
      NativeNumeric::F32(v) => Param::F32(Box::new(F32{inner: v})),
      NativeNumeric::I64(v) => Param::I64(Box::new(I64{inner: v})),
      NativeNumeric::F64(v) => Param::F64(Box::new(F64{inner: v})),

    }
  }
}


impl ExecuterTrait for Executer{
  fn handle_inst(self: &mut Self, inst: &Instruction)->Result<ExecutionRes, ExecutionErr>{
    match inst.op_code{
      OpCode::I32Const =>{
        // let result:Result<I32,_> = inst.params.try_into();
        // TODO: error handling
        let stack_element: StackElement = (inst.params[0].clone()).into();
        self.stack.push(stack_element);
        return Ok(ExecutionRes{});
      }
      OpCode::End => {
        return Err(ExecutionErr::Terminate);
      }
      _ => {
        panic!("op code not implemented");
      }
    }
  }

  fn run(self: &mut Self, program: &Vec<Instruction>){
    loop{
      let result = self.handle_inst(&program[self.pc]);
      match result{
        Ok(_)=>{
          self.pc += 1;
        },
        Err(ExecutionErr::Terminate)=>{
          let final_result = self.stack.pop();
          if final_result.is_err(){
            println!("Program terminated without return value");
          } else{
            let final_value = final_result.unwrap();
            println!("Return: {:#?}", final_value);
          }
          println!("Program terminated successfully.");
          break;
        }
        Err(ExecutionErr::Trap)=>{
          println!("Program trap.");
          break;
        }
      }
    }
  }
  fn run_function(self: &mut Self, module: &Module, index: usize) {
    let code: &Code = &module.codes[index];
    let result = parse_code(code);
    // push a stack frame
    let af_meta: AfMeta = AfMeta { len: code.local_var_types.len() as u8, reference: get_af_references(&code.local_var_types) };
    let af = ActivationFrame{
      index: self.af_meta.len() as u8,
      locals: get_af_locals(&code.local_var_types)
    };
    self.af_meta.push(af_meta);
    self.stack.push(StackElement::Activation(Box::new(af)));
    self.run(&result);
  }
}

fn get_af_references(local_var_types: &Vec<u8>)->Vec<u8>{
  let mut ret: Vec<u8> = vec![];
  let mut ref_index = 0;
  for i in 0..local_var_types.len(){
    ret.push(ref_index);
    let size = get_type_size(&local_var_types[i].try_into().unwrap());
    ref_index += size as u8;
  }
  ret
}
fn get_af_locals(local_var_types: &Vec<u8>)->Vec<u8>{
  let mut ret: Vec<u8> = vec![];
  for i in 0..local_var_types.len() {
    let size = get_type_size(&local_var_types[i].try_into().unwrap());
    ret.push(local_var_types[i]); // type
    let mut temp = vec![0;size]; // data
    ret.append(&mut temp);
  }
  ret
}

fn parse_code(code: &Code)->Vec<Instruction>{
  let mut res: Vec<Instruction> = vec![];
  let mut i = 0;
  while i < code.body.len() {
    let result: Result<OpCode, ()> = code.body[i].try_into();
    if result.is_err(){
      println!("{}", i);
      println!("Error parsing instruction");
      panic!();
    }
    let op = result.unwrap();
    i += 1;
    match op{
      OpCode::I32Const=>{
        let mut buf_reader = BufReader::new(&code.body[i..]);
        let result = i32::seg(&mut buf_reader);
        if result.is_err() {
          println!("Error parsing i32");
          panic!();
        }
        let parameter_LEB = result.unwrap();
        i += parameter_LEB.len();
        let parameter_native = LEB_to_native(&parameter_LEB, &ValueType::I32);
        if parameter_native.is_err(){
          println!("Error parsing I32Const");
          panic!();
        }
        let native = parameter_native.unwrap();
        res.push(Instruction { op_code: op, params: vec![native.into()]});
      }
      OpCode::End=>{
        res.push(Instruction { op_code: op, params: vec![]});
      }
      _ => {
        panic!("can not parse")
      }
    }
  }
  res
}