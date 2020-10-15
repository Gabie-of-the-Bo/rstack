#[macro_use] 
extern crate lazy_static;

mod instruction;
mod machine;
mod parser;
mod input;
mod output;

use machine::*;
use instruction::*;
use input::*;

fn main() {
    let mut machine = Machine::new();

    let path: String = String::from("resources/fib3.rstack");
    let program: Program = parser::parse_file(&path);

    let mut i = InputBuffer::<Vec<u8>>::from_vector_u32(vec![410000]);
    let mut o = output::buffers::null();
    
    //machine.run(&program, &i, &o);
    machine.debug(&program, &mut i, &mut o, false, true);
}
