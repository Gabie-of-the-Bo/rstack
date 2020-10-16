extern crate regex;

use crate::instruction::*;
use std::collections::HashMap;

pub fn to_instruction(vector: &Vec<&str>) -> Instruction{
    let p = |i: &str| {i.parse::<u32>().unwrap()};

    return match vector.as_slice(){
        ["CONST", a] => Instruction::from((InstId::CONST, p(a))),

        ["IN32"] => Instruction::from(InstId::IN32),
        ["IN16"] => Instruction::from(InstId::IN16),
        ["IN8"] => Instruction::from(InstId::IN8),

        ["STORE"] => Instruction::from(InstId::STORE),
        ["STOREC", a] => Instruction::from((InstId::STOREC, p(a))),
        ["STOREAT", a] => Instruction::from((InstId::STOREAT, p(a))),

        ["FETCH"] => Instruction::from(InstId::FETCH),
        ["FETCHFROM", a] => Instruction::from((InstId::FETCHFROM, p(a))),

        ["ADD"] => Instruction::from(InstId::ADD),
        ["SUB"] => Instruction::from(InstId::SUB),
        ["MUL"] => Instruction::from(InstId::MUL),
        ["DIV"] => Instruction::from(InstId::DIV),
        ["ADDF"] => Instruction::from(InstId::ADDF),
        ["SUBF"] => Instruction::from(InstId::SUBF),
        ["MULF"] => Instruction::from(InstId::MULF),
        ["DIVF"] => Instruction::from(InstId::DIVF),

        ["JMP", a] => Instruction::from((InstId::JMP, p(a))),
        ["BRZ"] => Instruction::from(InstId::BRZ),
        ["BRNZ"] => Instruction::from(InstId::BRNZ),

        ["CALL"] => Instruction::from(InstId::CALL),
        ["CALLC", a] => Instruction::from((InstId::CALLC, p(a))),
        ["RET"] => Instruction::from(InstId::RET),

        ["DUP"] => Instruction::from(InstId::DUP),
        ["PEEK", a] => Instruction::from((InstId::PEEK, p(a))),
        ["ROTL", a] => Instruction::from((InstId::ROTL, p(a))),
        ["ROTR", a] => Instruction::from((InstId::ROTR, p(a))),

        ["OUT"] => Instruction::from(InstId::OUT),

        _ => Instruction::from(InstId::HALT)
    };
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
