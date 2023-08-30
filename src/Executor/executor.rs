use super::stack::*;
use super::utils::{get_type_size, LEB_to_native};
use super::value::*;
use super::value::{ValueType, F32, F64, I32, I64};
use crate::instruction::OpCode;
use crate::Loader::consts::Type;
use crate::Loader::walker::{
    walk, walk_str, walk_with_delimiter, walk_with_size, Segable, Walkable,
};
use crate::Module::module::{Code, Function, Module};
use std::collections::HashMap;
use std::{convert::TryFrom, io::BufReader, io::Read};
fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}
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
struct Store {}

struct BlockInfo {
    start: u32, // start instruction index
    end: u32,   // end instruction index
                // TODO: block type, arity, etc
}

struct FunctionInfo {
    // for now, use a different name from BlockInfo, although they are of similar usage
    start: u32,
    end: u32,
}
pub trait ExecutorTrait {
    fn handle_inst(self: &mut Self, inst: &Instruction, terminal: u32) -> Result<ExecutionRes, ExecutionErr>;
    fn run(self: &mut Self, terminal: u32);
    fn run_function(self: &mut Self, index: u32);
}
pub struct Executor<'a> {
    pc: u32,
    stack: Stack,
    store: Store,
    af_meta: Vec<AfMeta>,
    pub module: &'a Module,
    block_meta: HashMap<u32, BlockInfo>, // block start idx -> BlockInfo
    function_meta: Vec<FunctionInfo>,    // function_idx -> FunctionInfo
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
pub struct Instruction {
    op_code: OpCode,
    params: Vec<Param>,
}

#[derive(Debug)]
pub enum Param {
    I32(I32),
    I64(I64),
    F32(F32),
    F64(F64),
    V128(V128),
}

pub struct ExecutionRes {
    // data: Vec<u8>,
    // value_types: Vec<ValueType>
}
pub enum ExecutionErr {
    Trap,
    Terminate,
}

impl From<StackElement> for Param {
    fn from(value: StackElement) -> Self {
        match value {
            StackElement::I32(v) => Param::I32(I32 { inner: (*v).inner }),
            StackElement::I64(v) => Param::I64(I64 { inner: (*v).inner }),
            StackElement::F32(v) => Param::F32(F32 { inner: (*v).inner }),
            StackElement::F64(v) => Param::F64(F64 { inner: (*v).inner }),
            _ => panic!("type converstion error"),
        }
    }
}

impl From<Param> for StackElement {
    fn from(value: Param) -> Self {
        match value {
            Param::I32(v) => StackElement::I32(Box::new(v)),
            Param::I64(v) => StackElement::I64(Box::new(v)),
            Param::F32(v) => StackElement::F32(Box::new(v)),
            Param::F64(v) => StackElement::F64(Box::new(v)),
            Param::V128(v) => StackElement::V128(Box::new(v)),
            _ => panic!("type converstion error"),
        }
    }
}

impl Clone for Param {
    fn clone(&self) -> Self {
        match self {
            Param::I32(v) => Param::I32(I32 { inner: (*v).inner }),
            Param::I64(v) => Param::I64(I64 { inner: (*v).inner }),
            Param::F32(v) => Param::F32(F32 { inner: (*v).inner }),
            Param::F64(v) => Param::F64(F64 { inner: (*v).inner }),
            Param::V128(v) => Param::V128(V128 {
                inner: (*v).inner.to_vec(),
            }),
        }
    }
}

impl From<NativeNumeric> for Param {
    fn from(value: NativeNumeric) -> Self {
        match value {
            NativeNumeric::I32(v) => Param::I32(I32 { inner: v }),
            NativeNumeric::F32(v) => Param::F32(F32 { inner: v }),
            NativeNumeric::I64(v) => Param::I64(I64 { inner: v }),
            NativeNumeric::F64(v) => Param::F64(F64 { inner: v }),
        }
    }
}

impl<'a> Executor<'a> {
    // start_idx: the base index of the instruction to be parsed
    // it is primarily used to register blocks
    fn parse_code(&mut self, code: &Code, start_idx: u32) -> Vec<Instruction> {
        // mantain a stack to trace block scope. Each entry is the start index of a block
        let mut block_stack: Vec<u32> = vec![];
        let mut res: Vec<Instruction> = vec![];
        let mut i = 0;
        let mut inst_count = 0;
        while i < code.body.len() {
            let result: Result<OpCode, ()> = code.body[i].try_into();
            if result.is_err() {
                println!("{}", code.body[i]);
                println!("Error parsing instruction");
                panic!();
            }
            let op = result.unwrap();
            i += 1;
            let mut buf_reader = BufReader::new(&code.body[i..]);
            match op {
                OpCode::I32Const => {
                    let result = i32::seg(&mut buf_reader);
                    if result.is_err() {
                        println!("Error parsing");
                        panic!();
                    }
                    let parameter_LEB = result.unwrap();
                    i += parameter_LEB.len();
                    let parameter_native = LEB_to_native(&parameter_LEB, &ValueType::I32);
                    if parameter_native.is_err() {
                        println!("Error parsing I32Const");
                        panic!();
                    }
                    let native: NativeNumeric = parameter_native.unwrap();
                    res.push(Instruction {
                        op_code: op,
                        params: vec![native.into()],
                    });
                }
                OpCode::Call => {
                    //FIXME: should be unsigned
                    //FIXME: LEB
                    let result = i8::walk(&mut buf_reader);
                    i += 1;
                    if result.is_err() {
                        panic!("Error parsing");
                    }
                    let function_idx = result.unwrap();
                    res.push(Instruction {
                        op_code: op,
                        params: vec![Param::I32(I32 {
                            inner: function_idx.into(),
                        })],
                    });
                }
                OpCode::LocalSet | OpCode::LocalGet => {
                    // params: [index_of_local_var]

                    //FIXME: should be unsigned
                    //FIXME: LEB
                    let result = i8::walk(&mut buf_reader);
                    i += 1;
                    if result.is_err() {
                        panic!("Error parsing");
                    }
                    let localidx = result.unwrap();

                    res.push(Instruction {
                        op_code: op,
                        params: vec![Param::I32(I32 {
                            inner: localidx.into(),
                        })],
                    });
                }
                OpCode::Unreachable | OpCode::Nop => {
                    // have to push otherwise the instruction index does not match
                    res.push(Instruction {
                        op_code: op,
                        params: vec![],
                    });
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
                        num_u8 => {
                            params.push(Param::I32(I32 {
                                inner: i32::from(num_u8),
                            }));
                        }
                        _ => panic!("Error parsing"),
                    }
                    // push block
                    block_stack.push(start_idx + inst_count);
                    // TODO: to unify function and block, we can first push a start_idx to the block_stack
                    res.push(Instruction {
                        op_code: op,
                        params: params,
                    });
                }
                OpCode::End => {
                    res.push(Instruction {
                        op_code: op,
                        params: vec![],
                    });
                    let result = block_stack.pop();
                    if result.is_some() {
                        let start_index = result.unwrap();
                        self.block_meta.insert(
                            start_index,
                            BlockInfo {
                                start: start_index,
                                end: start_idx + inst_count,
                            },
                        );
                    }
                }
                OpCode::I32Eq | OpCode::I32Ne | OpCode::I32Or | OpCode::I32Add => {
                    res.push(Instruction {
                        op_code: op,
                        params: vec![],
                    });
                }
                OpCode::BrIf => {
                    // params: [labelidx] branch out of labelidx(th) block
                    //FIXME: should be unsigned
                    //FIXME: LEB
                    let result = i8::walk(&mut buf_reader);
                    let label_idx = result.unwrap();
                    res.push(Instruction {
                        op_code: op,
                        params: vec![Param::I32(I32 {
                            inner: i32::from(label_idx),
                        })],
                    });
                }
                _ => {
                    panic!("can not parse")
                }
            }
            inst_count += 1;
        }
        res
    }

    pub fn parse_instructions(&mut self, module: &'a Module) {
        // concatenate instructions from all functions into a single vector for universal addressing
        // update the block bookkeeping
        let mut inst_idx = 0;
        for func_idx in 0..self.module.functions.len() {
            let code = &self.module.codes[func_idx as usize];
            let mut result = self.parse_code(code, inst_idx);
            // register function
            let func_meta = FunctionInfo {
                start: inst_idx,
                end: inst_idx + result.len() as u32,
            };
            self.function_meta.push(func_meta);
            // update instruction index
            inst_idx += result.len() as u32;
            self.instructions.append(&mut result);
        }
    }

    pub fn init(&mut self) {
        self.parse_instructions(self.module);
        for i in 0..self.instructions.len(){
          println!("{} {:#?}", i, self.instructions[i].op_code);
        }
        // other inititializations
    }
    pub fn new(module: &'a Module) -> Self {
        return Executor {
            pc: 0,
            stack: Default::default(),
            store: Default::default(),
            af_meta: Default::default(),
            block_meta: Default::default(),
            function_meta: Default::default(),
            instructions: vec![],
            module: module,
        };
    }
}

impl Executor<'_> {
    pub fn get_result(&mut self) -> StackElement {
      return self.stack.pop().unwrap();
    }
    fn get_current_af(&mut self) -> Result<&mut Box<ActivationFrame>, ()> {
        let len = self.af_meta.len();
        let current_af_meta = &mut self.af_meta[len - 1];
        if let Ok(StackElement::Activation(current_af_boxed)) =
            self.stack.get(current_af_meta.position)
        {
            return Ok(current_af_boxed);
        } else {
            return Err(());
        }
    }
    fn branch(self: &mut Self, label_idx: u32){
      /*
      let L be the l-th label appearing on the stack, starting from top, counting from 0
      let n be arity of L
      pop n values from the stack
      pop values from the stack until the lth label reveals, pop the label
      push n values from the stack
      */

      // TODO: pop n values from the stack
      for i in 0..label_idx+1{
        // label_idx + 1 because we count from 0
        // label_idx == 0, we still need to execute once
        loop {
          let top = self.stack.pop().unwrap();
          if let StackElement::Label(l) = top{
            if i == label_idx{
              self.pc = l.continuation; // jump to the continuation of the label
              // no need to increment one here because we will increment in the main loop
            }
            break;
          }
        }
      }
      // TODO: push n values to the stack
    }
}

impl ExecutorTrait for Executor<'_> {
    fn handle_inst(self: &mut Self, inst: &Instruction, terminal: u32) -> Result<ExecutionRes, ExecutionErr> {
        println!("{}:{:#?}", self.pc, inst.op_code);
        match inst.op_code {
            OpCode::I32Const => {
                // let result:Result<I32,_> = inst.params.try_into();
                // TODO: error handling
                let stack_element: StackElement = (inst.params[0].clone()).into();
                self.stack.push(stack_element);

                return Ok(ExecutionRes {});
            }
            OpCode::Call => {
                if let Param::I32(function_idx) = &inst.params[0] {
                    // FIXME:
                    self.run_function((*function_idx).inner.try_into().unwrap());
                }
                return Ok(ExecutionRes {});
            }
            OpCode::LocalGet => {
                if let Param::I32(local_idx) = &inst.params[0] {
                    let current_af = self.get_current_af().unwrap();
                    let value = (*current_af).locals[(*local_idx).inner as usize].clone();
                    let value2 = (*current_af).locals[(*local_idx).inner as usize].clone();
                    if let Param::I32(v) = value {
                      if v.inner < -3 {
                        panic!();
                      }
                    }
                    self.stack.push((value2).into());
                }
                return Ok(ExecutionRes {});
            }
            OpCode::LocalSet => {
                if let Param::I32(local_idx) = &inst.params[0] {
                    let value = self.stack.pop().unwrap();
                    let current_af = self.get_current_af().unwrap();
                    (*current_af).locals[(*local_idx).inner as usize] = value.clone().into();
                }
                return Ok(ExecutionRes {});
            }
            OpCode::Unreachable => {
              return Ok(ExecutionRes {})
            }
            OpCode::I32Or => {
              let top1 = self.stack.pop().unwrap();
              let top2 = self.stack.pop().unwrap();
              if let StackElement::I32(top1_value) = top1 {
                  if let StackElement::I32(top2_value) = top2 {
                      //FIXME: bitwise disjunction?
                      // println!("{} {}", (*top1_value).inner, (*top2_value).inner);
                      self.stack.push(StackElement::I32(Box::new(I32 {
                          inner: (*top1_value).inner | (*top2_value).inner,
                      })));
                  }
              }
              return Ok(ExecutionRes {});
            }
            OpCode::I32Eq | OpCode::I32Ne => {
              let top1 = self.stack.pop().unwrap();
              let top2 = self.stack.pop().unwrap();
              if let StackElement::I32(top1_value) = top1 {
                  if let StackElement::I32(top2_value) = top2 {
                      let mut value = 0;
                      if (*top1_value).inner == (*top2_value).inner && inst.op_code == OpCode::I32Eq {
                          value = 1;
                      }
                      if (*top1_value).inner != (*top2_value).inner && inst.op_code == OpCode::I32Ne {
                        value = 1;
                    }
                      self.stack
                          .push(StackElement::I32(Box::new(I32 { inner: value })));
                  }
              }
              return Ok(ExecutionRes {});
            }
            OpCode::I32Add => {
              let top1 = self.stack.pop().unwrap();
              let top2 = self.stack.pop().unwrap();
              if let StackElement::I32(top1_value) = top1 {
                  if let StackElement::I32(top2_value) = top2 {
                      let value =  (*top1_value).inner + (*top2_value).inner;
                      self.stack
                          .push(StackElement::I32(Box::new(I32 { inner: value })));
                  }
              }
              return Ok(ExecutionRes {});
            }
            OpCode::Block => {
              // push a label on the stack
              let result = self.block_meta.get(&self.pc).unwrap();

              let label = Label{
                arity: 0,  //FIXME: real arity
                continuation: result.end,
                target: (result.start, result.end),
              };
              self.stack.push(StackElement::Label(Box::new(label)));
              return Ok(ExecutionRes {});
            }
            OpCode::BrIf => {
                if let Param::I32(label_idx) = &inst.params[0] {
                  // pop a value from the stack
                  if let StackElement::I32(value) = self.stack.pop().unwrap(){
                    if (*value).inner != 0{
                      // non-zero, execute the instruction `branch to label`
                      // TODO: implement arity
                      self.branch(label_idx.inner as u32);
                    }
                  }
                }
                return Ok(ExecutionRes {});
            }
            OpCode::Loop => {
              // FIXME: arity, pop m value from the stack and then enter
              // push L to the stack
              let result = self.block_meta.get(&self.pc).unwrap();
              let label = Label{
                arity: 0, // FIXME:
                continuation: result.start,
                target: (result.start, result.end), // continuation is the start of the loop
              };
              self.stack.push(StackElement::Label(Box::new(label)));
              return Ok(ExecutionRes {});
            }
            OpCode::End => {
                // Exiting from a block
                /*
                pop all values val from the top of the stack until label L
                pop label from the stack
                push all values val back to the stack
                jump to the end of the structured control instruction associated with label L
                 */
                let mut buffer:Vec<StackElement> = vec![];
                loop {
                  let top = self.stack.pop().unwrap();
                  match top {
                    StackElement::Label(l)=>{
                        self.pc = l.target.1;
                      // if self.pc > terminal{
                      //   // out of the function
                      //   return Ok(ExecutionRes {  });
                      // }
                      break;
                    }
                    _ => {
                      buffer.push(top);
                    }
                  }
                }
                for _j in 0..buffer.len(){
                  self.stack.push(buffer.pop().unwrap());
                }
                return Ok(ExecutionRes {  });
            }
            _ => {
                panic!("op code not implemented");
            }
        }
    }

    fn run(self: &mut Self, terminal: u32) {
        println!("!!+++++++++++++++Execution Start+++++++++++++++++++!!");
        // for i in 0..self.instructions.len(){
        //   println!("{}:{:#?}", i, self.instructions[i].op_code);
        // }
        //TODO: fix me
        loop {
            let inst = self.instructions[self.pc as usize].clone();
            let result = self.handle_inst(&inst, terminal);
            match result {
                Ok(_) => {
                    self.pc += 1;
                }
                Err(ExecutionErr::Terminate) => {
                    let final_result = self.stack.pop();
                    if final_result.is_err() {
                        println!("Program terminated without return value");
                    } else {
                        let final_value = final_result.unwrap();
                        println!("Return: {:#?}", final_value);
                    }
                    println!("Program terminated successfully.");
                    break;
                }
                Err(ExecutionErr::Trap) => {
                    println!("Program trap.");
                    break;
                }
            }
            if self.pc >= terminal {
              break;
            }
        }
        println!("!!+++++++++++++++Execution End+++++++++++++++++++!!");

    }
    fn run_function(self: &mut Self, index: u32) {
        let code = &self.module.codes[index as usize];
        // original pc
        let caller_pc = self.pc;
        // jumpt to function
        self.pc = self.function_meta[index as usize].start;
        // get the local arguments
        let func_type_index = self.module.functions[index as usize].type_index;
        let func_param_types = &self.module.function_types[func_type_index as usize].0;
        let func_return_types = &self.module.function_types[func_type_index as usize].1;
        let func_param_count = func_param_types.len();
        // pop from the stack and put into the stack frame
        let mut params: Vec<Param> = vec![];
        for _i in 0..func_param_count {
            let stack_element = self.stack.pop().unwrap();
            params.push(stack_element.into());
        }
        // push a stack frame
        let af_meta: AfMeta = AfMeta {
            len: code.local_var_types.len() as u8,
            position: self.stack.size(),
        };
        let af = ActivationFrame {
            index: self.af_meta.len() as u8,
            arity: func_return_types.len() as u8,
            locals: get_af_locals(&params, &code.local_var_types),
        };
        self.af_meta.push(af_meta);
        self.stack.push(StackElement::Activation(Box::new(af)));

        // push a label
        let function_meta = &self.function_meta[index as usize];
        let label: Label = Label { arity: func_return_types.len() as u8, continuation: caller_pc, target: (function_meta.start, function_meta.end) };
        self.stack.push(StackElement::Label(Box::new(label)));

        self.run(self.function_meta[index as usize].end);
        // function returns,

        // get arity, i.e. restore the local variables
        let af = self.get_current_af().unwrap();
        let arity = (*af).arity;
        let mut buffer:Vec<StackElement> = vec![];
        for _j in 0..arity{
          let top = self.stack.pop().unwrap();
          buffer.push(top);
        }
        // pop frame from the stack
        let top =  self.stack.pop().unwrap();
        match top{
          StackElement::Activation(_) => {
            self.af_meta.pop();
          }
          _ => {
            // print_type_of(&top);
            println!("{:#?}", top);
            panic!("function returns but no activation frame");
          }
        }
        for _j in 0..arity{
          let p = buffer.pop().unwrap();
          self.stack.push(p);
        }
        self.pc = caller_pc;
    }
}

fn get_af_locals(params: &Vec<Param>, local_var_types: &Vec<Type>) -> Vec<Param> {
    let mut ret: Vec<Param> = params.to_vec();
    for i in 0..local_var_types.len() {
        // println!("{}", &local_var_types[i]);
        let value_type: &ValueType = &local_var_types[i].try_into().unwrap();
        match value_type {
            ValueType::I32 => {
                ret.push(Param::I32(I32 { inner: 0 }));
            }
            ValueType::I64 => {
                ret.push(Param::I64(I64 { inner: 0 }));
            }
            ValueType::F32 => {
                ret.push(Param::F32(F32 { inner: 0.0 }));
            }
            ValueType::F64 => {
                ret.push(Param::F64(F64 { inner: 0.0 }));
            }
            _ => panic!("not implemented"),
        }
    }
    ret
}
