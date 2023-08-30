mod Loader;
mod Module;
mod instruction;
mod Executor;
use Loader::loader::*;
use Executor::executor::*;
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
    if result.is_err() {
        println!("Failed to parse");
    }
    println!("{:#?}", module);

    let mut executor: Executor::executor::Executor = Executor::executor::Executor::new(&module);
    executor.module = &module;
    executor.init();
    executor.run_function(1 as u32);
    println!("Result: {:#?}", executor.get_result());
}
