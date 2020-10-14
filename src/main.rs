mod instruction;
mod machine;
mod parser;
mod input;
mod output;

use machine::*;
use instruction::*;
use input::*;
use output::*;

fn main() {
    let mut machine = Machine::new();

    let path: String = String::from("resources/fib3.rstack");
    let program: Program = parser::parse_file(&path);

    let mut i = InputBuffer::<Vec<u8>>::from_vector_u32(vec![41]);
    let mut o = OutputBuffer::new_printer();
    
    //machine.run(&program, &i, &o);
    machine.debug(&program, &mut i, &mut o, false, true);
}
