// walkers
use std::io::BufReader;
use std::io::Read;
use std::mem;
use std::u8;

//LEB-128 walker implementation
pub trait Walkable{
  fn walk<R>(buf_reader: &mut BufReader<R>) ->Result<Self, std::io::Error> where R: std::io::Read, Self: Sized;
}

pub trait Segable{
  fn seg<R>(buf_reader: &mut BufReader<R>) -> Result<Vec<u8>, std::io::Error> where R: std::io::Read, Self: Sized;
}

impl Segable for i32{
  fn seg<R>(buf_reader: &mut BufReader<R>) -> Result<Vec<u8>, std::io::Error> where R: std::io::Read, Self: Sized{
    let mut buffer = [0];
    let mut result = Vec::new();
    let mut value:i32;
    let mut shift = 0;
    loop{
      buf_reader.read_exact(&mut buffer)?;
      value = buffer[0] as i32;
      result.push(buffer[0]);
      shift += 7;
      if shift >= 32 || value & 0x80 == 0{
        break;
      }
    }
    Ok(result)
  }
}


impl Segable for u32{
  fn seg<R>(buf_reader: &mut BufReader<R>) -> Result<Vec<u8>, std::io::Error> where R: std::io::Read, Self: Sized{
    let mut buffer = [0];
    let mut value:u32;
    let mut result = Vec::new();
    loop{
      buf_reader.read_exact(&mut buffer)?;
      value = buffer[0] as u32;
      result.push(buffer[0]);
      if value & 0x80 == 0{
        break;
      }
    }
    Ok(result)
  }
}

impl Walkable for i32{
  fn walk<R>(buf_reader: &mut BufReader<R>)->Result<i32, std::io::Error> where R: std::io::Read{
    let mut buffer = [0];
    let mut value:i32;
    let mut result:i32 = 0;
    let mut shift = 0;
    loop{
      buf_reader.read_exact(&mut buffer)?;
      value = buffer[0] as i32;
      result |= (value & 0x7f) << shift;
      shift += 7;
      if shift >= 32 || value & 0x80 == 0{
        break;
      }
    }
    if shift < 32 && buffer[0] & 0x40 != 0{
      result |= -(1i32 << shift);
    }
    Ok(result)
  }
}

impl Walkable for u32{
  fn walk<R>(buf_reader: &mut BufReader<R>)->Result<u32, std::io::Error> where R: std::io::Read{
    let mut buffer = [0];
    let mut value:u32;
    let mut result:u32 = 0;
    let mut shift = 0;
    loop{
      buf_reader.read_exact(&mut buffer)?;
      value = buffer[0] as u32;
      result |= (value & 0x7f) << shift;
      shift += 7;
      if value & 0x80 == 0{
        break;
      }
    }
    Ok(result)
  }
}


impl Walkable for i8{
  fn walk<R>(buf_reader: &mut BufReader<R>)->Result<i8, std::io::Error> where R: std::io::Read{
    let mut buffer = [0; mem::size_of::<i8>()];
    buf_reader.read_exact(&mut buffer)?;
    let i = i8::from_le_bytes(buffer);
    Ok(i)
  }
}

impl Walkable for u8{
  fn walk<R>(buf_reader: &mut BufReader<R>)->Result<u8, std::io::Error> where R: std::io::Read{
    let mut buffer = [0; mem::size_of::<u8>()];
    buf_reader.read_exact(&mut buffer)?;
    let i = u8::from_le_bytes(buffer);
    Ok(i)
  }
}

pub fn walk_with_size<R>(buf_reader: &mut BufReader<R>, size: usize) -> Result<Vec<u8>, std::io::Error> where R: std::io::Read{
  let mut res: Vec<u8> = vec![];
  for _i in 0..size{
    res.push(u8::walk(buf_reader)?);
  }
  Ok(res)
}


pub fn walk<T,R>(buf_reader: &mut BufReader<R>)-> Result<T, std::io::Error> where R: std::io::Read, T:Walkable {
  return T::walk::<R>(buf_reader);
}

pub fn walk_with_delimiter<R>(buf_reader: &mut BufReader<R>, delim: u8)->Result<Vec<u8>, std::io::Error> where R: std::io::Read{
  let mut res: Vec<u8> = vec![];
  let mut current = u8::walk(buf_reader)?;
  res.push(current);
  while current != delim {
    current = u8::walk(buf_reader)?;
    res.push(current);
  }
  Ok(res)
}



pub fn walk_str<R>(buf_reader: &mut BufReader<R>, len: usize) -> Result<String, std::io::Error> where R: std::io::Read{
  let mut buffer = vec![0; len];
  buf_reader.take(len as u64).read_exact(&mut buffer)?;
  Ok(String::from_utf8(buffer).unwrap())
  // FIXME: Error handling
}
#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_walk_i8(){
    let data = vec![0xf9];
    let mut reader = BufReader::new(data.as_slice());
    let result = i8::walk(&mut reader);
    assert_eq!(result.unwrap(), -7);
  }

  // fn test_walk_u8(){
  //   let data = vec![0xf9];
  //   let mut reader = BufReader::new(data.as_slice());
  //   let result = u8::walk(&mut reader);
  //   assert_eq!(result.unwrap(), 249 as u8);
  // }

  // fn test_walk_i32(){
  //   let data = vec![0xf9; 4];
  //   let mut reader = BufReader::new(data.as_slice());
  //   let result = i32::walk(&mut reader);
  //   assert_eq!(result.unwrap(), -101058055);
  // }

  // fn test_walk_u32(){
  //   let data = vec![0xf9; 4];
  //   let mut reader = BufReader::new(data.as_slice());
  //   let result = u32::walk(&mut reader);
  //   assert_eq!(result.unwrap(), 4193909241 as u32);
  // }

  // fn test_walk_str(){
    
  // }
}



