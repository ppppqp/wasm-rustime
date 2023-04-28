mod Loader;
mod Module;
mod instruction;
use Loader::loader;
use std::fs::File;
fn main() {
    let loader: Loader::loader::Loader;
    let inputFile = File::open("program.wasm")?;
    let result = loader.validate(inputFile);
    // let result = "hello";
    println!("{}", result);
}
