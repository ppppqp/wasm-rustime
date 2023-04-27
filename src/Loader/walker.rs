// walkers
use std::io::BufReader;
use std::io::Read;
use std::mem;
use std::u8;

pub fn walki32<R>(buf_reader: &mut BufReader<R>)->Result<i32, std::io::Error> where R: std::io::Read{
  let mut buffer = [0; mem::size_of::<i32>()];
  buf_reader.read_exact(&mut buffer)?;
  let i = i32::from_be_bytes(buffer);
  Ok(i)
}

pub fn walku32<R>(buf_reader: &mut BufReader<R>)->Result<u32, std::io::Error> where R: std::io::Read{
  let mut buffer = [0; mem::size_of::<u32>()];
  buf_reader.read_exact(&mut buffer)?;
  let i = u32::from_be_bytes(buffer);
  Ok(i)
}

pub fn walki8<R>(buf_reader: &mut BufReader<R>)->Result<i8, std::io::Error> where R: std::io::Read{
  let mut buffer = [0; mem::size_of::<i8>()];
  buf_reader.read_exact(&mut buffer)?;
  let i = i8::from_be_bytes(buffer);
  Ok(i)
}

pub fn walku8<R>(buf_reader: &mut BufReader<R>)->Result<u8, std::io::Error> where R: std::io::Read{
  let mut buffer = [0; mem::size_of::<u8>()];
  buf_reader.read_exact(&mut buffer)?;
  let i = u8::from_be_bytes(buffer);
  Ok(i)
}

pub fn walk<T,R>(buf_reader: &mut BufReader<R>)->Result<u8, std::io::Error> where R: std::io::Read, T:Sized {
  match std::any::type_name::<T>(){
    "u32" => {
      return Ok(walku32(buf_reader)?);
    }
    "i32" => {
      return Ok(walki32(buf_reader)?);
    }
    "u8" => {
      return Ok(walku8(buf_reader)?);
    }
    "i8" => {
      return Ok(walki8(buf_reader)?);
    }
  }
}

pub fn walkWithDelimiter<T, R>(buf_reader: &mut BufReader<R>, delim: T)->Result<Vec<T>, std::io::Error> where R: std::io::Read{
  let res: Vec<T> = vec![];
  let mut current = walk<T>(buf_reader)?;
  res.push(current);
  while current != delim {
    current = walk<T>(buf_reader)?;
    res.push(current);
  }
  res;
}

pub fn walkStr<R>(buf_reader: &mut BufReader<R>, len: usize) -> Result<String, std::io::Error> where R: std::io::Read{
  let mut buffer = vec![0; len];
  buf_reader.take(len as u64).read_exact(&mut buf)?;
  Ok(String::from_utf8(buffer)?)
}
#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_walk_i8(){
    let data = vec![0xf9];
    let mut reader = BufReader::new(data.as_slice());
    let result = walki8(&mut reader);
    assert_eq!(result.unwrap(), -7);
  }

  fn test_walk_u8(){
    let data = vec![0xf9];
    let mut reader = BufReader::new(data.as_slice());
    let result = walku8(&mut reader);
    assert_eq!(result.unwrap(), 249 as u8);
  }

  fn test_walk_i32(){
    let data = vec![0xf9; 4];
    let mut reader = BufReader::new(data.as_slice());
    let result = walki32(&mut reader);
    assert_eq!(result.unwrap(), -101058055);
  }

  fn test_walk_u32(){
    let data = vec![0xf9; 4];
    let mut reader = BufReader::new(data.as_slice());
    let result = walku32(&mut reader);
    assert_eq!(result.unwrap(), 4193909241 as u32);
  }

  fn test_walk_str(){
    
  }
}



