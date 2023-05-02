use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;


#[derive(Debug)]
pub struct Import{
  pub module: String,
  pub field: String,
  pub description: u8
}

#[derive(Debug)]
pub struct Function{
  pub index: u32
}
#[derive(Debug)]
pub struct Export{
  pub field: String,
  pub description: u8,
  pub index: u32,
}
#[derive(Debug)]
pub struct Table{
  pub element_type: u8,
  pub limit_flag: u8,
  pub limit_initial: u32,
  pub limit_max: u32,
}
#[derive(Debug)]
pub struct Memory{
  pub limit_flag: u8,
  pub limit_initial: u32,
  pub limit_max: u32,
}
#[derive(Debug)]
pub struct Global{
  pub value_type: u8,
  pub mutable: u8,
  pub init_expr: Vec<u8>
}
#[derive(Debug)]
pub struct Code{
  pub body: Vec<u8>,
  pub local_var_types: Vec<u8>,
}
#[derive(Debug)]
pub struct Element{
  pub table_index: u32,
  pub init_expr: Vec<u8>,
  pub func_indices: Vec<u32>,
}
#[derive(Debug)]
pub struct Data{
  pub memory_index: u32,
  pub init_expr: Vec<u8>,
  pub data: Vec<u8>
}

#[derive(Default, Debug)]
pub struct Module{
  pub start_index: u32, 
  pub function_types: Vec<(Vec<u32>, Vec<u32>)>, 
  pub imports: Vec<Import>, 
  pub exports: Vec<Export>, 
  pub functions: Vec<Function>, 
  pub tables: Vec<Table>, 
  pub memories: Vec<Memory>, 
  pub codes: Vec<Code>, 
  pub globals: Vec<Global>, 
  pub elements: Vec<Element>, 
  pub data: Vec<Data>
}
