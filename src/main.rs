mod Loader;
mod Module;
mod instruction;
use Loader::loader::*;
use Module::module::*;
use std::fs::File;
fn main() {
    let loader: Loader::loader::Loader = Default::default();
    let result = File::open("program.wasm");
    if result.is_err(){
        println!("Failed to open input file");
    }
    let file = result.unwrap();
    let mut module: Module::module::Module = Default::default();
    let result = loader.parse(file, &mut module);
    // let result = "hello";
    println!("{:#?}", module);
}
