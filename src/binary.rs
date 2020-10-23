use std::convert::TryInto;

use crate::instruction::*;

#[allow(dead_code)]
pub fn serialize_program(program: &Program) -> Vec<u8>{
    return program.iter().map(|i| i.into()).flat_map(|i: Vec<u8>| i).collect()
}

#[allow(dead_code)]
pub fn save_program(program: &Program, path: String){
    let bytes: Vec<u8> = serialize_program(&program);
    std::fs::write(path, bytes).expect("Error while writing file");
}

#[allow(dead_code)]
pub fn load_program_from_bytes(bytes: &Vec<u8>) -> Program{
    let mut res: Program = vec!();

    let mut i = 0;

    while i < bytes.len(){
        let id: InstId = bytes[i].try_into().expect("Invalid instruction ID");
        let mut args: Vec<u32> = vec!();

        i += 1;

        for _ in 0..id.arg_number(){
            let arg: [u8; 4] = bytes[i..(i + 4)].try_into().expect("Invalid number of bytes");
            args.push(u32::from_be_bytes(arg));
            i += 4;
        }

        while args.len() < 2{
            args.push(0);
        }

        res.push(Instruction::from((id, args[0], args[1])));
    }

    return res;    
}

#[allow(dead_code)]
pub fn load_program(path: String) -> Program{
    let bytes: Vec<u8> = std::fs::read(path).expect("Error while reading file");
    
    return load_program_from_bytes(&bytes);
}