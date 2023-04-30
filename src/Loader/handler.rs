
// handlers
use std::io::{BufReader, Seek};
use super::walker::{walk, walkStr, walkWithDelimiter, walkWithSize};
use super::consts::*;
use crate::instruction::OpCode;
use crate::Module::module::Module;
use crate::Module::module::*;
pub trait Handler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read + Seek;
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
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read{
    Ok(())
  }
}

impl Handler for TypeHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read{
    let _ = walk::<u32, R>(buf_reader)?;
    let entity_count = walk::<u32, R>(buf_reader)?;
    for _i in [0..entity_count]{
      if walk::<u32, R>(buf_reader)? != Type::Func as u32{
        // TODO: expection terminate
      }

      let mut funcType: (Vec<u32>, Vec<u32>) = (vec![], vec![]);
      let paramCount = walk::<u32, R>(buf_reader)?;
      for _j in [0..paramCount]{
        funcType.0.push(walk::<u32, R>(buf_reader)?);
      }
      
      let resultCount: u32 = walk::<u32, R>(buf_reader)?;
      for _j in [0..resultCount]{
        funcType.1.push(walk::<u32, R>(buf_reader)?);
      }

      module.funcTypes.push(funcType); 
    }

    Ok(())
  }
}

impl Handler for ImportHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let importEntityCount = walk::<u32, R>(buf_reader)?;
      for _i in [0..importEntityCount]{
        let moduleNameLen = walk::<u32, R>(buf_reader)?;
        let moduleNameStr = walkStr(buf_reader, moduleNameLen as usize)?;
        let fieldLen = walk::<u32, R>(buf_reader)?;
        let fieldStr = walkStr(buf_reader, fieldLen as usize)?;
        let description = walk::<u8, R>(buf_reader)?;
        module.imports.push(Import{module: moduleNameStr, field: fieldStr, description: description});
      }
      Ok(())
  }
}

impl Handler for ExportHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let exportCount = walk::<u32, R>(buf_reader)?;
      for _i in [0..exportCount]{
        let fieldLen = walk::<u32, R>(buf_reader)?;
        let fieldStr = walkStr(buf_reader, fieldLen as usize)?;
        let description = walk::<u8, R>(buf_reader)?;
        let index = walk::<u32, R>(buf_reader)?;
        module.exports.push(Export{field:fieldStr, description:description, index: index});
      }
      Ok(())
  }
}

impl Handler for FunctionHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let functionCount = walk::<u32, R>(buf_reader)?;
      for _i  in [0..functionCount]{
        let functionIndex = walk::<u32, R>(buf_reader)?;
        module.functions.push(Function{index: functionIndex});  
      }
      Ok(())
  }
}

impl Handler for TableHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader);
      let tableCount = walk::<u32, R>(buf_reader)?;
      for _i in [0..tableCount]{
        let elementType = walk::<u8, R>(buf_reader)?;
        let limitFlag = walk::<u8, R>(buf_reader)?;
        let limitInitial = walk::<u32, R>(buf_reader)?;
        let mut table: Table = Table{elementType, limitFlag: limitFlag, limitInitial, limitMax: 0};
        if limitFlag == LimitFlag::ACTIVE as u8{
          table.limitMax = walk::<u32, R>(buf_reader)?;
        }
        module.tables.push(table);
      }
      Ok(())
  }
}

impl Handler for MemoryHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let memoryCount = walk::<u32, R>(buf_reader)?;
      for _i in [0..memoryCount]{
        let limitFlag = walk::<u8, R>(buf_reader)?;
        let limitInitial = walk::<u32, R>(buf_reader)?;
        let mut memory: Memory = Memory{limitFlag, limitInitial, limitMax: 0};
        if limitFlag == LimitFlag::ACTIVE as u8{
          memory.limitMax = walk::<u32, R>(buf_reader)?;
        }
        module.memories.push(memory);
      }
      Ok(())
  }
}

impl Handler for StartHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
    let startIndex = walk::<u32, R>(buf_reader)?;
    module.startFnIndex = startIndex;
    Ok(())
  }
}

impl Handler for GlobalHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader)?;
      let globalCount = walk::<u32, R>(buf_reader)?;
      for _i in [0..globalCount]{
        let valueType = walk::<u8, R>(buf_reader)?;
        let mutable = walk::<u8, R>(buf_reader)?;
        let initExpr = walkWithDelimiter::<u8, R>(buf_reader, OpCode::End as u8)?;
        module.globals.push(Global{valueType, mutable, initExpr});
      }
      Ok(())
  }
}

impl Handler for CodeHandler {
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read + Seek {
      let _ = walk::<u32, R>(buf_reader)?;
      let funcDefCount = walk::<u32, R>(buf_reader)?;
      for _i in [0..funcDefCount]{
        let bodySize = walk::<u32, R>(buf_reader)?;
        let localVarStart: u32 = buf_reader.stream_position()? as u32;
        let totalTypesNum = walk::<u32, R>(buf_reader)?;
        let mut localVarTypes:Vec<u8> = vec![];
        for _j in [0..totalTypesNum]{
          let varCount = walk::<u32, R>(buf_reader)?;
          let varType = walk::<u8, R>(buf_reader)?;
          let mut temp = vec![varType; varCount as usize];
          localVarTypes.append(&mut temp);
        }
        let localVarEnd: u32 = buf_reader.stream_position()? as u32;
        let body = walkWithSize::<u8, R>(buf_reader, (bodySize - (localVarEnd - localVarStart)) as usize)?;
        module.codes.push(Code{localVarTypes, body});
      }
      Ok(())
  }
}

impl Handler for ElementHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32, R>(buf_reader);
      let elementCount = walk::<u32, R>(buf_reader)?;
      for _i in [0..elementCount]{
        let tableIndex = walk::<u32, R>(buf_reader)?;
        let initExpr = walkWithDelimiter::<u8, R>(buf_reader, OpCode::End as u8)?;
        let funcIndicesCount = walk::<u32, R>(buf_reader)?;
        let mut funcIndices: Vec<u32> = vec![];
        for _j in [0..funcIndicesCount]{
          funcIndices.push(walk::<u32, R>(buf_reader)?);
        }
        module.elements.push(Element{tableIndex, initExpr, funcIndices});
      }
      Ok(())
  }
}

impl Handler for DataHandler {
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
    let _ = walk::<u32, R>(buf_reader);
    let dataSegCount = walk::<u32, R>(buf_reader)?;
    for _i in [0..dataSegCount]{
      let memoryIndex = walk::<u32, R>(buf_reader)?;
      let initExpr = walkWithDelimiter::<u8, R>(buf_reader, OpCode::End as u8)?;
      let size = walk::<u32, R>(buf_reader)?;
      let data = walkWithSize::<u8, R>(buf_reader, size as usize)?;
      module.data.push(Data{memoryIndex, initExpr, data});
    }
    Ok(())
  }
}

impl Handler for CustomHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
    let _ = walk::<u32, R>(buf_reader);
    let size = walk::<u32, R>(buf_reader)?;
    walkWithSize::<u8, R>(buf_reader, size as usize)?;
    Ok(())
  }
}