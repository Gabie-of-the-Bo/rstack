use std::convert::TryInto;

use crate::instruction::*;

#[allow(dead_code)]
pub fn serialize_program(program: &Program) -> Vec<u8>{
    let mut res: Vec<u8> = vec!();
    let max_arg = program.iter().map(|i| std::cmp::max(i.args[0], i.args[1])).max().or_else(|| Some(0)).unwrap();
    
    match max_arg{
        i if i < u8::MAX as u32 => {
            res.push(8);
            program.iter().map(|i| i.to_bytes_u8()).flat_map(|i: Vec<u8>| i).for_each(|i| res.push(i))
        },

        i if i < u16::MAX as u32 => {
            res.push(16);
            program.iter().map(|i| i.to_bytes_u16()).flat_map(|i: Vec<u8>| i).for_each(|i| res.push(i))
        },

        _ => {
            res.push(32);
            program.iter().map(|i| i.to_bytes_u32()).flat_map(|i: Vec<u8>| i).for_each(|i| res.push(i))
        },
    };

    return res;
}

#[allow(dead_code)]
pub fn save_program(program: &Program, path: String){
    let bytes: Vec<u8> = serialize_program(&program);
    std::fs::write(path, bytes).expect("Error while writing file");
}

#[allow(dead_code)]
pub fn load_program_from_bytes(bytes: &Vec<u8>) -> Program{
    let mut res: Program = vec!();
    let width: u8 = bytes[0];

    let mut i = 1;

    while i < bytes.len(){
        let id: InstId = bytes[i].try_into().expect("Invalid instruction ID");
        let mut args: Vec<u32> = vec!();

        i += 1;

        match width{
            8 => {
                for _ in 0..id.arg_number(){
                    args.push(bytes[i] as u32);
                    i += 1;
                }
            },
            
            16 => {
                for _ in 0..id.arg_number(){
                    let arg: [u8; 2] = bytes[i..(i + 2)].try_into().expect("Invalid number of bytes");
                    args.push(u16::from_be_bytes(arg) as u32);
                    i += 2;
                }
            },

            32 => {
                for _ in 0..id.arg_number(){
                    let arg: [u8; 4] = bytes[i..(i + 4)].try_into().expect("Invalid number of bytes");
                    args.push(u32::from_be_bytes(arg));
                    i += 4;
                }
            }
            _ => unreachable!()
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