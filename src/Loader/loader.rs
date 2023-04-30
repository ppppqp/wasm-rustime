
use std::io::{Read, BufReader, Seek};
use super::walker::{walk};
use super::handler::{
  Handler,
  CustomHandler,
  HeaderHandler,
  TypeHandler,
  ImportHandler,
  ExportHandler,
  FunctionHandler,
  TableHandler,
  MemoryHandler,
  StartHandler,
  GlobalHandler,
  CodeHandler,
  ElementHandler,
  DataHandler, 
};
use crate::Module::module::*;
// load a wasm file
#[derive(Default)]
pub struct Loader{
  customHandler: CustomHandler,
  headerHandler: HeaderHandler,
  typeHandler: TypeHandler,
  importHandler: ImportHandler,
  exportHandler: ExportHandler,
  functionHandler: FunctionHandler,
  tableHandler: TableHandler,
  memoryHandler: MemoryHandler,
  startHandler: StartHandler,
  globalHandler: GlobalHandler,
  elementHandler: ElementHandler,
  codeHandler: CodeHandler,
  dataHandler: DataHandler,
}

#[derive(Debug)]
pub enum ValidateError{
  ErrorReadingBytes(std::io::Error),
  ErrorMagicBytes,
  ErrorVersionNumber,
}

pub trait Load{
  fn validate<R:Read>(&self, data:R)->Result<(), ValidateError>;
  fn parse<R:Read>(&self, data:R, module: &mut Module) where R: Read + Seek;
}




impl Load for Loader{
  fn validate<R:Read>(&self, data:R)->Result<(), ValidateError> {
    let mut reader = BufReader::new(data);

    let mut res = walk::<u32, R>(&mut reader);
    if let Err(err) = res{
      return Err(ValidateError::ErrorReadingBytes(err));
    }
    if res.unwrap() != 0x6D736100{
      return Err(ValidateError::ErrorMagicBytes);
    }

    res = walk::<u32, R>(&mut reader);
    if let Err(err) = res{
      return Err(ValidateError::ErrorReadingBytes(err));
    }
    if res.unwrap() != 0x1{
      return Err(ValidateError::ErrorVersionNumber)
    }
    Ok(())
  }


  fn parse<R>(&self, data:R, module: &mut Module) where R: Read + Seek{
    let mut buf_reader = BufReader::new(data);
    while buf_reader.buffer().len() > 0 {
      let sectionId = walk::<u8, R>(&mut buf_reader).unwrap();
      match sectionId {
        0=>{ self.customHandler.handle(&mut buf_reader, module);}
        1=>{ self.typeHandler.handle(&mut buf_reader,module); }
        2=>{ self.importHandler.handle(&mut buf_reader, module); }
        3=>{ self.functionHandler.handle(&mut buf_reader, module); }
        4=>{ self.tableHandler.handle(&mut buf_reader, module); }
        5=>{ self.memoryHandler.handle(&mut buf_reader, module); }
        6=>{ self.globalHandler.handle(&mut buf_reader, module); }
        7=>{ self.exportHandler.handle(&mut buf_reader, module); }
        8=>{ self.startHandler.handle(&mut buf_reader, module); }
        9=>{ self.elementHandler.handle(&mut buf_reader, module); }
        10=>{ self.codeHandler.handle(&mut buf_reader, module); }
        11=>{ self.dataHandler.handle(&mut buf_reader, module); }
        12=>{ todo!(); }
        _ => { todo!();}
      }
    }
  }
}


