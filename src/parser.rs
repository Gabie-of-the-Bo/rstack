extern crate regex;

use crate::instruction::*;

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
    }

    let program_str = std::fs::read_to_string(path).expect("Error while reading file");
    
    fn trim_comments(l: &'_ str) -> String{
        return COMMENTS_RE.replace(l, "${1}").into_owned();
    }

    fn separate_args(l: &String) -> Vec<&str>{
        return l.trim().split_whitespace().collect();
    }

    return program_str.to_uppercase().lines()
                .filter(|l| !l.is_empty())
                .map(trim_comments)
                .map(|l| to_instruction(&separate_args(&l)))
                .collect();
}
