pub struct Import{
  module: String,
  field: String,
  description: u8
}

pub struct Function{
  index: u32
}
pub struct Export{
  field: String,
  description: u8,
  index: u32,
}

pub struct Table{
  elementType: u8,
  limitFlag: u8,
  limitInitial: u32,
  limitMax: u32,
}

pub struct Memory{
  limitFlag: u8,
  limitInitial: u32,
  limitMax: u32,
}

pub struct Global{
  limitFlag: u8,
  limitInitial: u32,
  initExpr: Vec<u8>
}

pub struct Module{
  pub startFnIndex: u32,
  pub funcTypes: Vec<(Vec<i32>, Vec<i32>)>,
  pub imports: Vec<Import>,
  pub exports: Vec<Export>,
  pub functions: Vec<Function>,
  pub tables: Vec<Table>
}

