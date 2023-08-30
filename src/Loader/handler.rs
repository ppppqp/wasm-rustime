
// handlers
use std::io::{BufReader, Seek};
use super::walker::{walk, walk_str, walk_with_delimiter, walk_with_size};
use super::consts::*;
use crate::instruction::OpCode;
use crate::Module::module::Module;
use crate::Module::module::*;
pub enum HandlerError{
  ErrorReadingBytes(std::io::Error),
  InvalidFuncType,
}

impl From<std::io::Error> for HandlerError {
  fn from(e: std::io::Error) -> Self {
      Self::ErrorReadingBytes(e)
  }
}
pub trait Handler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read + Seek;
}
#[derive(Default)]
pub struct HeaderHandler{}
#[derive(Default)]
pub struct TypeHandler{}
#[derive(Default)]
pub struct ImportHandler{}
#[derive(Default)]
pub struct ExportHandler{}
#[derive(Default)]
pub struct FunctionHandler{}
#[derive(Default)]
pub struct TableHandler{}
#[derive(Default)]
pub struct MemoryHandler{}
#[derive(Default)]
pub struct StartHandler{}
#[derive(Default)]
pub struct ElementHandler{}
#[derive(Default)]
pub struct GlobalHandler{}
#[derive(Default)]
pub struct CodeHandler{}
#[derive(Default)]
pub struct DataHandler{}
#[derive(Default)]
pub struct CustomHandler{}

impl Handler for HeaderHandler{
  fn handle<R>(&self, _buf_reader: &mut BufReader<R>, _module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read{
    Ok(())
  }
}

impl Handler for TypeHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read{
    let _ = walk::<u32, R>(buf_reader)?;
    let entity_count = walk::<u32, R>(buf_reader)?;
    for _i in 0..entity_count{
      if walk::<u32, R>(buf_reader)? != Type::Func as u32{
        return Err(HandlerError::InvalidFuncType);
      }

      let mut func_type: (Vec<u32>, Vec<u32>) = (vec![], vec![]);
      let param_count = walk::<u32, R>(buf_reader)?;

      for _j in 0..param_count{
        func_type.0.push(walk::<u32, R>(buf_reader)?);
      }
      
      let result_count: u32 = walk::<u32, R>(buf_reader)?;
      for _j in 0..result_count{
        func_type.1.push(walk::<u32, R>(buf_reader)?);
      }
      module.function_types.push(func_type); 
    }
    println!("Type section passed");
    Ok(())
  }
}

impl Handler for ImportHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let import_entity_count = walk::<u32, R>(buf_reader)?;
      for _i in 0..import_entity_count{
        let module_name_len = walk::<u32, R>(buf_reader)?;
        let module_name_str = walk_str(buf_reader, module_name_len as usize)?;
        let field_len = walk::<u32, R>(buf_reader)?;
        let field_str = walk_str(buf_reader, field_len as usize)?;
        let description = walk::<u8, R>(buf_reader)?;
        module.imports.push(Import{module: module_name_str, field: field_str, description: description});
      }
      println!("Import section passed");
      Ok(())
  }
}

impl Handler for ExportHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let export_count = walk::<u32, R>(buf_reader)?;
      for _i in 0..export_count{
        let field_len = walk::<u32, R>(buf_reader)?;
        let field_str = walk_str(buf_reader, field_len as usize)?;
        let description = walk::<u8, R>(buf_reader)?;
        let index = walk::<u32, R>(buf_reader)?;
        module.exports.push(Export{field:field_str, description:description, index: index});
      }
      println!("Export section passed");
      Ok(())
  }
}

impl Handler for FunctionHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let function_count = walk::<u32, R>(buf_reader)?;
      for _i  in 0..function_count{
        let type_index = walk::<u32, R>(buf_reader)?;
        module.functions.push(Function{type_index: type_index});  
      }
      println!("Function section passed");
      Ok(())
  }
}

impl Handler for TableHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader);
      let table_count = walk::<u32, R>(buf_reader)?;
      for _i in 0..table_count{
        let element_type = walk::<u8, R>(buf_reader)?;
        let limit_flag = walk::<u8, R>(buf_reader)?;
        let limit_initial = walk::<u32, R>(buf_reader)?;
        let mut table: Table = Table{element_type, limit_flag, limit_initial, limit_max: 0};
        if limit_flag == LimitFlag::ACTIVE as u8{
          table.limit_max = walk::<u32, R>(buf_reader)?;
        }
        module.tables.push(table);
        println!("Table section passed");
      }
      Ok(())
  }
}

impl Handler for MemoryHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let memory_count = walk::<u32, R>(buf_reader)?;
      for _i in 0..memory_count{
        let limit_flag = walk::<u8, R>(buf_reader)?;
        let limit_initial = walk::<u32, R>(buf_reader)?;
        let mut memory: Memory = Memory{limit_flag, limit_initial, limit_max: 0};
        if limit_flag == LimitFlag::ACTIVE as u8{
          memory.limit_max = walk::<u32, R>(buf_reader)?;
        }
        module.memories.push(memory);
      }
      println!("Memory section passed");
      Ok(())
  }
}

impl Handler for StartHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
    let start_index = walk::<u32, R>(buf_reader)?;
    module.start_index = start_index;
    println!("Start section passed");
    Ok(())
  }
}

impl Handler for GlobalHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let global_count = walk::<u32, R>(buf_reader)?;
      for _i in 0..global_count{
        let value_type = walk::<u8, R>(buf_reader)?;
        let mutable = walk::<u8, R>(buf_reader)?;
        let init_expr = walk_with_delimiter::<R>(buf_reader, OpCode::End as u8)?;
        module.globals.push(Global{ value_type, mutable, init_expr});
      }
      println!("Global section passed");
      Ok(())
  }
}

impl Handler for CodeHandler {
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read + Seek {
      let _ = walk::<u32, R>(buf_reader)?;
      let func_def_count = walk::<u32, R>(buf_reader)?;
      for _i in 0..func_def_count{
        let body_size = walk::<u32, R>(buf_reader)?;
        let local_var_start: u32 = buf_reader.stream_position()? as u32;
        let total_types_num = walk::<u32, R>(buf_reader)?;
        let mut local_var_types:Vec<Type> = vec![];
        for _j in 0..total_types_num{
          let var_count: u32 = walk::<u32, R>(buf_reader)?;
          let var_type = walk::<u8, R>(buf_reader)?;
          let mut temp:Vec<Type> = vec![var_type.try_into().unwrap(); var_count as usize];
          local_var_types.append(&mut temp);
        }
        let local_var_end: u32 = buf_reader.stream_position()? as u32;
        let body = walk_with_size::<R>(buf_reader, (body_size - (local_var_end - local_var_start)) as usize)?;
        module.codes.push(Code{local_var_types, body});
      }
      println!("Code section passed");
      Ok(())
  }
}

impl Handler for ElementHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader);
      let element_count = walk::<u32, R>(buf_reader)?;
      for _i in 0..element_count{
        let table_index = walk::<u32, R>(buf_reader)?;
        let init_expr = walk_with_delimiter::<R>(buf_reader, OpCode::End as u8)?;
        let func_indices_count = walk::<u32, R>(buf_reader)?;
        let mut func_indices: Vec<u32> = vec![];
        for _j in 0..func_indices_count{
          func_indices.push(walk::<u32, R>(buf_reader)?);
        }
        module.elements.push(Element{table_index, init_expr, func_indices});
      }
      println!("Element section passed");
      Ok(())
  }
}

impl Handler for DataHandler {
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
    let _ = walk::<u32, R>(buf_reader);
    let data_seg_count = walk::<u32, R>(buf_reader)?;
    for _i in 0..data_seg_count{
      let memory_index = walk::<u32, R>(buf_reader)?;
      let init_expr = walk_with_delimiter::<R>(buf_reader, OpCode::End as u8)?;
      let size = walk::<u32, R>(buf_reader)?;
      let data = walk_with_size::<R>(buf_reader, size as usize)?;
      module.data.push(Data{memory_index, init_expr, data});
    }
    println!("Data section passed");
    Ok(())
  }
}

impl Handler for CustomHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, _: &mut Module)-> Result<(), HandlerError> where R: std::io::Read {
    let _ = walk::<u32, R>(buf_reader);
    let size = walk::<u32, R>(buf_reader)?;
    walk_with_size::<R>(buf_reader, size as usize)?;
    println!("Custom section passed");
    Ok(())
  }
}