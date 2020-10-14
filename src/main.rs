mod instruction;
mod machine;
mod parser;
mod input;

use machine::*;
use instruction::*;
use input::*;

fn main() {
    let mut machine = Machine::new();

    let path: String = String::from("resources/fib2.stack");
    let program: Program = parser::parse_file(&path);

    let mut i = InputBuffer::<Vec<u8>>::from_vector_u32(vec![41]);
    
    //machine.run(&program, i);
    machine.debug(&program, &mut i, false, true);
}
