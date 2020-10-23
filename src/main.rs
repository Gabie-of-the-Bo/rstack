use rstack::*;

fn main() {
    let mut machine = machine::Machine::new();

    let path: String = String::from("resources/fib3.rstack");
    let mut program: instruction::Program = parser::parse_file(&path);

    binary::save_program(&program, "resources/bin.crstack".to_string());
    program = binary::load_program("resources/bin.crstack".to_string());

    let mut i = input::InputBuffer::<Vec<u8>>::from_vector_u32(vec![10]);
    let mut o = output::buffers::printer();
    
    //machine.run(&program, &i, &o);
    machine.debug(&program, &mut i, &mut o, false, true);
}