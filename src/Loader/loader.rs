
use std::io::{Read, BufReader};
use super::walker::{walki32, walku32, walki8, walku8};
use super::handler::{
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
// load a wasm file
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
  dataHandler: DataHandle,
  dataCountHandler: DataCountHandler,
}

pub enum ValidateError{
  ErrorReadingBytes(std::io::Error),
  ErrorMagicBytes,
  ErrorVersionNumber,
}

pub trait Load{
  fn validate<R:Read>(&self, data:R)->Result<(), ValidateError>;
  fn parse<R:Read>(&self, data:R);
}




impl Load for Loader{
  pub fn validate<R:Read>(&self, data:R)->Result<(), ValidateError> {
    let mut reader = BufReader::new(data);

    let mut res = walku32(&mut reader);
    if let Err(err) = res{
      return Err(ValidateError::ErrorReadingBytes(err));
    }
    if res.unwrap() != 0x6D736100{
      return Err(ValidateError::ErrorMagicBytes);
    }

    res = walku32(&mut reader);
    if let Err(err) = res{
      return Err(ValidateError::ErrorReadingBytes(err));
    }
    if res.unwrap() != 0x1{
      return Err(ValidateError::ErrorVersionNumber)
    }
    Ok(())
  }


  fn parse<R:Read>(&self, data:R, module: Module){
    let mut reader = BufReader::new(data);
    while BufReader.buffer().len() > 0 {
      let sectionId = walk::<u8>(reader)?;
      match sectionId {
        0=>{ self.customHandler.handle(buf_reader, module);}
        1=>{ self.typeHandler.handle(buf_reader, module); }
        2=>{ self.importHandler.handle(buf_reader, module); }
        3=>{ self.functionHandler.handle(buf_reader, module); }
        4=>{ self.tableHandler.handle(buf_reader, module); }
        5=>{ self.memoryHandler.handle(buf_reader, module); }
        6=>{ self.globalHandler.handle(buf_reader, module); }
        7=>{ self.exportHandler.handle(buf_reader, module); }
        8=>{ self.startHandler.handle(buf_reader, module); }
        9=>{ self.elementHandler.handle(buf_reader, module); }
        10=>{ self.codeHandler.handle(buf_reader, module); }
        11=>{ self.dataHandler.handle(buf_reader, module); }
        12=>{ todo!(); }
      }
    }
  }
}


