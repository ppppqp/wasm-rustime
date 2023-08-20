mod Loader;
mod Module;
mod instruction;
mod Executer;
use Loader::loader::*;
use Module::module::*;
use Executer::executor::*;
use std::fs::File;
fn main() {
    let loader: Loader::loader::Loader = Default::default();
    let result = File::open("fib.wasm");
    if result.is_err(){
        println!("Failed to open input file");
    }
    let file = result.unwrap();
    let mut module: Module::module::Module = Default::default();
    let result = loader.parse(file, &mut module);
    // let result = "hello";
    if result.is_err() {
        println!("Failed to parse");
    }
    println!("{:#?}", module);

    // let mut executer: Executer::executor::Executer = Default::default();
    // executer.run_function(&module, module.start_index as usize);
}
