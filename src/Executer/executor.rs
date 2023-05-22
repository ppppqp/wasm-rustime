use crate::Loader::walker::{Walkable, Segable, walk, walk_str, walk_with_delimiter, walk_with_size};
use std::{convert::TryFrom, io::BufReader, io::Read};
use crate::instruction::{OpCode};
use super::stack::{Stack, Stackish};
use super::value::{I32, I64, F32, F64, ValueType};
use crate::Module::module::{Module, Function, Code};



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
}

pub struct Instruction{
  opCode: OpCode,
  params: Vec<u8>
}


pub struct ExecutionRes{
  // data: Vec<u8>,
  // value_types: Vec<ValueType>
}
pub enum ExecutionErr{
  Trap,
  Terminate
}

impl ExecuterTrait for Executer{
  fn handle_inst(self: &mut Self, inst: &Instruction)->Result<ExecutionRes, ExecutionErr>{
    match inst.opCode{
      OpCode::I32Const =>{
        // let result:Result<I32,_> = inst.params.try_into();
        // TODO: error handling
        self.stack.push(ValueType::I32, &inst.params);
        return Ok(ExecutionRes{});
      }
      OpCode::End => {
        return Err(ExecutionErr::Terminate);
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
    //TODO: local variable
    let result = parse_code(code);
    self.run(&result);
  }
}

fn parse_code(code: &Code)->Vec<Instruction>{
  let mut res: Vec<Instruction> = vec![];
  let mut i = 0;
  while i < code.body.len() {
    let result: Result<OpCode, ()> = code.body[i].try_into();
    if result.is_err(){
      println!("Error parsing instruction");
      panic!();
    }
    let op = result.unwrap();
    match op{
      OpCode::I32Const=>{
        let mut buf_reader = BufReader::new(&code.body[i+1..]);
        let result = i32::seg(&mut buf_reader);
        if result.is_err() {
          println!("Error parsing i32");
          panic!();
        }
        let parameter = result.unwrap();
        i+= parameter.len();
        res.push(Instruction { opCode: op, params: parameter});
      }
      OpCode::End=>{
        res.push(Instruction { opCode: op, params: vec![]});
      }
    }
    i += 1;
  }
  res
}