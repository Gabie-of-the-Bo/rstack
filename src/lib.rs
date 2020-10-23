#[macro_use] 
extern crate lazy_static;

#[macro_use]
extern crate strum_macros;

use wasm_bindgen::prelude::*;

pub mod instruction;
pub mod machine;
pub mod parser;
pub mod input;
pub mod output;
pub mod binary;

use machine::*;
use binary::*;
use parser::*;
use input::*;

#[wasm_bindgen]
pub fn get_rstack_bin(code: &str) -> Vec<u8>{
    let program = parse_str(&code.to_owned());
    return serialize_program(&program);
}

#[wasm_bindgen]
pub fn execute_rstack_bytes(code: Vec<u8>) -> Vec<u32>{
    let mut machine = Machine::new();

    let program = load_program_from_bytes(&code);
    let mut result: Vec<u32> = vec!();

    {
        let mut i = InputBuffer::<Vec<u8>>::from_vector_u32(vec![10]);
        let mut o = output::buffers::storer(&mut result);
        
        machine.run(&program, &mut i, &mut o);
    }

    return result;
}

#[wasm_bindgen]
pub fn execute_rstack_str(code: &str) -> Vec<u32>{
    let mut machine = Machine::new();

    let program = parse_str(&code.to_owned());
    let mut result: Vec<u32> = vec!();

    {
        let mut i = InputBuffer::<Vec<u8>>::from_vector_u32(vec![10]);
        let mut o = output::buffers::storer(&mut result);
        
        machine.run(&program, &mut i, &mut o);
    }

    return result;
}