use crate::Loader::walker::{Walkable, Segable, walk, walk_str, walk_with_delimiter, walk_with_size};
use std::{convert::TryFrom, io::BufReader, io::Read};
use crate::instruction::{OpCode};
use super::stack::*;
use super::value::{I32, I64, F32, F64, ValueType};
use super::value::*;
use crate::Loader::consts::{Type};
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


pub trait ExecutorTrait{
  fn handle_inst(self: &mut Self, inst: &Instruction)->Result<ExecutionRes, ExecutionErr>;
  fn run(self: &mut Self, program: &Vec<Instruction>);
  fn run_function(self: &mut Self, index: u32);
  }
pub struct Executor<'a>{
  pc: usize,  
  stack: Stack,
  store: Store,
  af_meta: Vec<AfMeta>,
  pub module: &'a Module
}

pub struct Instruction{
  op_code: OpCode,
  params: Vec<Param>
}

pub enum Param{
  I32(I32),
  I64(I64),
  F32(F32),
  F64(F64),
  V128(V128),
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
          StackElement::I32(v)  => Param::I32(I32{inner: (*v).inner}),
          StackElement::I64(v)  => Param::I64(I64{inner: (*v).inner}),
          StackElement::F32(v)  => Param::F32(F32{inner: (*v).inner}),
          StackElement::F64(v)  => Param::F64(F64{inner: (*v).inner}),
          _ => panic!("type converstion error"),
      }
  }
}

impl From<Param> for StackElement{
  fn from(value: Param) -> Self {
      match value {
          Param::I32(v)  => StackElement::I32(Box::new(v)),
          Param::I64(v)  => StackElement::I64(Box::new(v)),
          Param::F32(v)  => StackElement::F32(Box::new(v)),
          Param::F64(v)  => StackElement::F64(Box::new(v)),
          Param::V128(v)  => StackElement::V128(Box::new(v)),
          _ => panic!("type converstion error"),
      }
  }
}

impl Clone for Param{
  fn clone(&self) -> Self {
      match self{
        Param::I32(v)  => Param::I32(I32{inner: (*v).inner}),
        Param::I64(v)  => Param::I64(I64{inner: (*v).inner}),
        Param::F32(v)  => Param::F32(F32{inner: (*v).inner}),
        Param::F64(v)  => Param::F64(F64{inner: (*v).inner}),
        Param::V128(v)  => Param::V128(V128{inner: (*v).inner.to_vec()})
      }
  }
}

impl From<NativeNumeric> for Param{
  fn from(value: NativeNumeric) -> Self {
    match value{
      NativeNumeric::I32(v) => Param::I32(I32{inner: v}),
      NativeNumeric::F32(v) => Param::F32(F32{inner: v}),
      NativeNumeric::I64(v) => Param::I64(I64{inner: v}),
      NativeNumeric::F64(v) => Param::F64(F64{inner: v}),

    }
  }
}

impl<'a> Executor<'a> {
  pub fn new(module: &'a Module) -> Self{
    return Executor{
      pc: 0,
      stack: Default::default(),
      store: Default::default(),
      af_meta: Default::default(),
      module: module
    };
  }
}

impl ExecutorTrait for Executor<'_>{
  fn handle_inst(self: &mut Self, inst: &Instruction)->Result<ExecutionRes, ExecutionErr>{
    match inst.op_code{
      OpCode::I32Const =>{
        // let result:Result<I32,_> = inst.params.try_into();
        // TODO: error handling
        let stack_element: StackElement = (inst.params[0].clone()).into();
        self.stack.push(stack_element);
        return Ok(ExecutionRes{});
      }
      OpCode::Call => {
        if let Param::I32(function_idx) = &inst.params[0]{
          self.run_function((*function_idx).inner.try_into().unwrap());
        }
        return Ok(ExecutionRes {  });
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
  fn run_function(self: &mut Self, index: u32) {
    let code: &Code = &self.module.codes[index as usize];
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



fn get_af_references(local_var_types: &Vec<Type>)->Vec<u8>{
  let mut ret: Vec<u8> = vec![];
  let mut ref_index = 0;
  for i in 0..local_var_types.len(){
    // println!("{}", local_var_types[i]);
    ret.push(ref_index);
    let size = get_type_size(&local_var_types[i].try_into().unwrap());
    ref_index += size as u8;
  }
  ret
}
fn get_af_locals(local_var_types: &Vec<Type>)->Vec<Box<Param>>{
  let mut ret: Vec<Box<Param>> = vec![];
  for i in 0..local_var_types.len() {
    // println!("{}", &local_var_types[i]);
    let value_type: &ValueType = &local_var_types[i].try_into().unwrap();
    match value_type {
      ValueType::I32 => {
        ret.push(Box::new(Param::I32(I32{inner: 0})));
      }
      ValueType::I64 => {
        ret.push(Box::new(Param::I64(I64{inner: 0})));
      }
      ValueType::F32 => {
        ret.push(Box::new(Param::F32(F32{inner: 0.0})));
      }
      ValueType::F64 => {
        ret.push(Box::new(Param::F64(F64{inner: 0.0})));
      }
      _ => panic!("not implemented")
    }
  }
  ret
}

fn parse_code(code: &Code)->Vec<Instruction>{
  let mut res: Vec<Instruction> = vec![];
  let mut i = 0;
  while i < code.body.len() {
    let result: Result<OpCode, ()> = code.body[i].try_into();
    if result.is_err(){
      println!("{}", code.body[i]);
      println!("Error parsing instruction");
      panic!();
    }
    let op = result.unwrap();
    i += 1;
    println!("op:{:#?}", op);
    let mut buf_reader = BufReader::new(&code.body[i..]);
    match op{
      OpCode::I32Const=>{
        let result = i32::seg(&mut buf_reader);
        if result.is_err() {
          println!("Error parsing");
          panic!();
        }
        let parameter_LEB = result.unwrap();
        i += parameter_LEB.len();
        let parameter_native = LEB_to_native(&parameter_LEB, &ValueType::I32);
        if parameter_native.is_err(){
          println!("Error parsing I32Const");
          panic!();
        }
        let native: NativeNumeric = parameter_native.unwrap();
        println!("{:?}", native);
        res.push(Instruction { op_code: op, params: vec![native.into()]});
      }
      OpCode::Call => {
        //FIXME: should be unsigned
        //FIXME: LEB
        let result = i8::walk(&mut buf_reader);
        i += 1;
        if result.is_err(){
          panic!("Error parsing");
        }
        let function_idx = result.unwrap();
        println!("{}", function_idx);
        res.push(Instruction{ op_code: op, params: vec![Param::I32(I32{inner: function_idx.into()})]});
      }
      OpCode::LocalSet | OpCode::LocalGet => {
        // params: [index_of_local_var]

        //FIXME: should be unsigned
        //FIXME: LEB
        let result = i8::walk(&mut buf_reader);
        i += 1;
        if result.is_err(){
          panic!("Error parsing");
        }
        let localidx = result.unwrap();
        println!("{}", localidx);

        res.push(Instruction{ op_code: op, params: vec![
          Param::I32(I32{inner: localidx.into()})
        ]});

      }
      OpCode::Unreachable | OpCode::Nop => {

      }
      OpCode::Block | OpCode::Loop => {
        // params: [block_type, ]
        // FIXME: should also consider the type index (i32 encoded in LEB128)
        let result = i8::walk(&mut buf_reader);
        i += 1;
        if result.is_err() {
          panic!("Error parsing");
        }
        let mut params: Vec<Param> = vec![];
        let result_u8 = result.unwrap();
        // FIXME: should also implement the cases for numtype, vectype, reftype, etc
        // let epsilon_u8: u8 = OpCode::BlockTypeE as u8;
        match result_u8 {
          num_u8 =>{
            params.push(Param::I32(I32{inner: i32::from(num_u8)}));
          }
          _ => panic!("Error parsing")
        }
      }
      OpCode::End =>{
        res.push(Instruction { op_code: op, params: vec![]});
      }
      OpCode::I32Eq | OpCode::I32Ne | OpCode::I32Or | OpCode::I32Add => {
        res.push(Instruction { op_code: op, params: vec![]});
      }
      OpCode::BrIf => {
        // params: [labelidx] branch out of labelidx(th) block
        //FIXME: should be unsigned
        //FIXME: LEB
        let result = i8::walk(&mut buf_reader);
        let label_idx = result.unwrap();
        println!("{}", label_idx);
        res.push(Instruction { op_code: op, params: vec![Param::I32(I32{inner: i32::from(label_idx)})]});
      }
      _ => {
        panic!("can not parse")
      }
    }
  }
  res
}