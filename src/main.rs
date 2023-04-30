mod Loader;
mod Module;
mod instruction;
use Loader::loader::*;

use std::fs::File;
fn main() {
    let loader: Loader::loader::Loader = Default::default();
    let result = File::open("../program.wasm");
    if result.is_err(){
        println!("Failed to open input file");
    }
    // let module: Module;
    let result = loader.validate(result.unwrap());
    if result.is_err(){
        println!("{:?}", result.unwrap_err());
    }
    // let result = "hello";
    // println!("{:?}", result);
}
