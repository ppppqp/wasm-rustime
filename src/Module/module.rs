pub struct Import{
  pub module: String,
  pub field: String,
  pub description: u8
}

pub struct Function{
  pub index: u32
}
pub struct Export{
  pub field: String,
  pub description: u8,
  pub index: u32,
}

pub struct Table{
  pub elementType: u8,
  pub limitFlag: u8,
  pub limitInitial: u32,
  pub limitMax: u32,
}

pub struct Memory{
  pub limitFlag: u8,
  pub limitInitial: u32,
  pub limitMax: u32,
}

pub struct Global{
  pub valueType: u8,
  pub mutable: u8,
  pub initExpr: Vec<u8>
}

pub struct Code{
  pub body: Vec<u8>,
  pub localVarTypes: Vec<u8>,
}

pub struct Element{
  pub tableIndex: u32,
  pub initExpr: Vec<u8>,
  pub funcIndices: Vec<u32>,
}

pub struct Data{
  pub memoryIndex: u32,
  pub initExpr: Vec<u8>,
  pub data: Vec<u8>
}
pub struct Module{
  pub startFnIndex: u32,
  pub funcTypes: Vec<(Vec<u32>, Vec<u32>)>,
  pub imports: Vec<Import>,
  pub exports: Vec<Export>,
  pub functions: Vec<Function>,
  pub tables: Vec<Table>,
  pub memories: Vec<Memory>,
  pub codes: Vec<Code>,
  pub globals: Vec<Global>,
  pub elements: Vec<Element>,
  pub data: Vec<Data>
}

