
use std::io::{Read, BufReader, Seek};
use super::walker::{walk};
use super::handler::{
  Handler,
  HandlerError,
  CustomHandler,
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
use crate::Loader::walker::walk_with_size;
use crate::Module::module::*;
// load a wasm file
#[derive(Default)]
pub struct Loader{
  custom_handler: CustomHandler,
  type_handler: TypeHandler,
  import_handler: ImportHandler,
  export_handler: ExportHandler,
  function_handler: FunctionHandler,
  table_handler: TableHandler,
  memory_handler: MemoryHandler,
  start_handler: StartHandler,
  global_handler: GlobalHandler,
  element_handler: ElementHandler,
  code_handler: CodeHandler,
  data_handler: DataHandler,
}

#[derive(Debug)]
pub enum ValidateError{
  ErrorReadingBytes(std::io::Error),
  ErrorMagicBytes,
  ErrorVersionNumber,
}

pub trait Load{
  fn validate<R:Read>(&self, buf_reader:&mut BufReader<R>)->Result<(), ValidateError>;
  fn parse<R>(&self, data:R, module: &mut Module)->Result<(), ParseError> where R: Read + Seek;
}

#[derive(Debug)]
pub enum ParseError{
  ErrorValidate(ValidateError),
  ErrorReadingBytes(std::io::Error),
  ErrorHandling,
}

impl From<HandlerError> for ParseError {
  fn from(e: HandlerError) -> Self {
      match e{
        HandlerError::ErrorReadingBytes(error) => {
          return ParseError::ErrorReadingBytes(error);
        }
        HandlerError::InvalidFuncType => {
          return ParseError::ErrorHandling;
        }
      }
      
  }
}

impl Load for Loader{
  fn validate<R:Read>(&self, buf_reader:&mut BufReader<R>)->Result<(), ValidateError> {
    let mut res = walk_with_size::<R>(buf_reader, 4);
    if let Err(err) = res{
      return Err(ValidateError::ErrorReadingBytes(err));
    }
    let magic = res.unwrap();

    if magic[0] != 0x00 || magic[1] != 0x61 || magic[2] != 0x73 || magic[3] != 0x6d{
      return Err(ValidateError::ErrorMagicBytes);
    }

    res = walk_with_size::<R>(buf_reader, 4);
    if let Err(err) = res{
      return Err(ValidateError::ErrorReadingBytes(err));
    }
    let version = res.unwrap();
    if version[0] != 0x01 || version[1] != 0x00 || version[2] != 0x00 || version[3] != 0x00{ 
      return Err(ValidateError::ErrorVersionNumber)
    }
    println!("Validation passed");
    Ok(())
  }


  fn parse<R>(&self, data:R, module: &mut Module)->Result<(), ParseError> where R: Read + Seek{
    let mut buf_reader = BufReader::new(data);
    let result = self.validate(&mut buf_reader);
    if result.is_err(){
      return Err(ParseError::ErrorValidate(result.unwrap_err()));
    }
    while buf_reader.buffer().len() > 0 {
      let section_id = walk::<u8, R>(&mut buf_reader).unwrap();
      match section_id {
        0=>{ self.custom_handler.handle(&mut buf_reader, module)?;}
        1=>{ self.type_handler.handle(&mut buf_reader,module)?; }
        2=>{ self.import_handler.handle(&mut buf_reader, module)?; }
        3=>{ self.function_handler.handle(&mut buf_reader, module)?; }
        4=>{ self.table_handler.handle(&mut buf_reader, module)?; }
        5=>{ self.memory_handler.handle(&mut buf_reader, module)?; }
        6=>{ self.global_handler.handle(&mut buf_reader, module)?; }
        7=>{ self.export_handler.handle(&mut buf_reader, module)?; }
        8=>{ self.start_handler.handle(&mut buf_reader, module)?; }
        9=>{ self.element_handler.handle(&mut buf_reader, module)?; }
        10=>{ self.code_handler.handle(&mut buf_reader, module)?; }
        11=>{ self.data_handler.handle(&mut buf_reader, module)?; }
        12=>{ todo!(); }
        _ => { todo!();}
      }
    }
    println!("All section passed");
    Ok(())
  }
}


