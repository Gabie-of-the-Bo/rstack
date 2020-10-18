extern crate regex;

use crate::instruction::*;
use std::collections::HashMap;

use std::str::FromStr;

#[allow(dead_code)]
pub fn print_program(program: &Program){
    for (line, code) in program.iter().enumerate(){
        println!("{}: {}", line, code);
    }
}

pub fn to_instruction(vector: &Vec<&str>) -> Instruction{
    let p = |i: &str| {i.parse::<u32>().unwrap()};
    
    let id = InstId::from_str(vector[0]).expect("Invalid instruction");
    let mut args = vector[1..].iter().map(|a| p(a)).collect::<Vec<u32>>();
    
    while args.len() < 2{
        args.push(0);
    }

    return Instruction::from((id, args[0], args[1]));
}

pub fn parse_file(path: &String) -> Program{
    lazy_static!{
        static ref COMMENTS_RE: regex::Regex = regex::Regex::new(r"^(.*?)(;.*)?$").unwrap();
        static ref LABELS_RE: regex::Regex = regex::Regex::new(r"^(.+):(.+)").unwrap();
    }

    fn get_label(l: (usize, &str)) -> (String, usize){
        if LABELS_RE.is_match(l.1){
            return (LABELS_RE.replace(l.1, "${1}").into_owned(), l.0);
        }

        return (String::new(), l.0);
    }
    
    let program_str = std::fs::read_to_string(path).expect("Error while reading file");
    let labels: HashMap<String, usize> = program_str.lines()
                                                    .filter(|l| !l.is_empty())
                                                    .enumerate()
                                                    .map(get_label)
                                                    .filter(|(i, _)| !i.is_empty())
                                                    .collect();
    
    let proprocess_code = |l: &str| -> String{
        let mut res = COMMENTS_RE.replace(l, "${1}").into_owned();

        if LABELS_RE.is_match(res.as_ref()){
            res = LABELS_RE.replace(res.as_ref(), "${2}").to_string();
        }

        for (i, j) in labels.iter(){
            res = res.replace(i, j.to_string().as_ref())
        }

        return res;
    };

    fn separate_args(l: &String) -> Vec<&str>{
        return l.trim().split_whitespace().collect();
    }

    return program_str.lines()
                      .filter(|l| !l.is_empty())
                      .map(proprocess_code)
                      .map(|l| to_instruction(&separate_args(&l)))
                      .collect();
}
