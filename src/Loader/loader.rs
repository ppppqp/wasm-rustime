
use std::io::{Read, BufReader};
use super::walker::{walki32, walku32, walki8, walku8};
use super::handler::{
  HeaderHandler,
  TypeHandler,
  ImportHandler,
  ExportHandler,
  FunctionHandler,
};
// load a wasm file
pub struct Loader{
  headerHandler: HeaderHandler,
  typeHandler: TypeHandler,
  importHandler: ImportHandler,
  exportHandler: ExportHandler,
  functionHandler: FunctionHandler,
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
  fn validate<R:Read>(&self, data:R)->Result<(), ValidateError> {
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


  fn parse<R:Read>(&self, data:R){
    let mut reader = BufReader::new(data);
    

  }
}


