
// handlers
use std::io::{BufReader};
use super::walker::{walk, self, walkStr, walkWithDelimiter};
use super::consts::*;
use crate::Module::module::Module;
pub trait Handler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read;
}

pub struct HeaderHandler{

}
pub struct TypeHandler{

}

pub struct ImportHandler{

}

pub struct ExportHandler{

}

pub struct FunctionHandler{

}

impl Handler for HeaderHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read{
    Ok(())
  }
}

impl Handler for TypeHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read{
    let _ = walk::<u32>(buf_reader)?;
    let entity_count = walk::<u32>(buf_reader)?;
    for _i in [0..entity_count]{
      if walki32(buf_reader)? != Type::Func as i32{
        // TODO: expection terminate
      }

      let mut funcType: (Vec<i32>, Vec<i32>);
      let paramCount = walk::<u32>(buf_reader)?;
      for _j in [0..paramCount]{
        funcType.0.push(walk::<u32>(buf_reader)?);
      }
      
      let resultCount: u32 = walk::<u32>(buf_reader)?;
      for _j in [0..resultCount]{
        funcType.1.push(walk::<u32>(buf_reader)?);
      }

      module.funcType.push(funcType); 
    }

    Ok(())
  }
}

impl Handler for ImportHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: &mut Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32>(&mut buf_reader)?;
      let importEntityCount = walk::<u32>(&mut buf_reader);
      for _i in [0..importEntityCount]{
        let moduleNameLen = walk::<u32>(&mut buf_reader)?;
        let moduleNameStr = walkStr(&mut buf_reader, moduleNameLen)?;
        let fieldLen = walk::<u32>(&mut buf_reader)?;
        let fieldStr = walkStr(&mut buf_reader, fieldLen)?;
        let description = walk::<u8>(&mut buf_reader)?;
        module.imports.push(Import{module: moduleNameStr, field: fieldStr, description: description});
      }
      Ok(())
  }
}

impl Handler for ExportHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32>(&mut buf_reader)?;
      let exportCount = walk::<u32>(&mut buf_reader)?;
      for _i in [0..exportCount]{
        let fieldLen = walk::<u32>(&mut buf_reader)?;
        let fieldStr = walkStr(&mut buf_reader, fieldLen)?;
        let description = walk::<u8>(&mut buf_reader)?;
        let index = walk::<u32>(&mut buf_reader)?;
        module.exports.push(Export{field:fieldStr, description:description, index: index});
      }
      Ok(())
  }
}

impl Handler for FunctionHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32>(buf_reader)?;
      let functionCount = walk::<u32>(buf_reader)?;
      for _i  in [0..functionCount]{
        let functionIndex = walk::<u32>(buf_reader)?;
        module.functions.push(Function{index: FunctionIndex});  
      }
      Ok(())
  }
}

impl Handler for TableHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32>(buf_reader);
      let tableCount = walk::<u32>(buf_reader)?;
      for _i in [0..tableCount]{
        let elementType = walk::<u8>(buf_reader)?;
        let limitFlag = walk::<u8>(buf_reader)?;
        let limitInitial = walk::<u8>(buf_reader)?;
        let mut table: Table = Table{elementType, limitFlag, limitInitial, limitMax: -1};
        if limitFlag == LimitFlag::ACTIVE{
          table.limitMax = walk::<u8>(buf_reader)?;
        }
        module.tables.push(table);
      }
      Ok(())
  }
}

impl Handler for MemoryHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32>(buf_reader)?;
      let memoryCount = walk::<u32>(buf_reader)?;
      for _i in [0..memoryCount]{
        let limitFlag = walk::<u8>(buf_reader)?;
        let limitInitial = walk::<u32>(buf_reader)?;
        let mut memory: Memory = Memory{limitFlag, limitInitial, limitMax: -1};
        if limitFlag == LimitFlag::ACTIVE{
          memory.limitMax = walk::<u8>(buf_reader)?;
        }
        module.memories.push(memory);
      }
      Ok(())
  }
}

impl Handler for StartHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read {
    let startIndex = walk::<u32>(buf_reader);
    module.startFnIndex = startIndex;
    Ok(())
  }
}

impl Handler for GlobalHandler{
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32>(buf_reader)?;
      let globalCount = walk::<u32>(buf_reader)?;
      for _i in [0..globalCount]{
        let valueType = walk::<u32>(buf_reader)?;
        let mutable = walk::<u32>(buf_reader)?;
        let initExpr = walkWithDelimiter::<u8>(buf_reader, OpCode::End);
        module.globals.push(Global{valueType, mutable, initExpr});
      }
      Ok(())
  }
}

impl Handler for CodeHandler {
  fn handle<R>(&self, buf_reader: &mut BufReader<R>, module: Module)-> Result<(), std::io::Error> where R: std::io::Read {
      let _ = walk::<u32>(buf_reader)?;
      let funcDefCount = walk::
      for _i in [0..globalCount]{
        let bodySize = walk::<u32>(buf_reader)?;
        let startPos = walk::<u32>(buf_reader)?;
        let localCount = walk::<u32>(buf_reader);
        let localVarType:Vec<u8> = vec![];
        for _j in [0..localCount]{
          localVarType.insert(0)
        }
      }
  }
}