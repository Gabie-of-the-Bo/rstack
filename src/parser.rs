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

        ["SHOW"] => Instruction::from(InstId::SHOW),

        _ => Instruction::from(InstId::HALT)
    };
}

pub fn parse_file(path: &String) -> Program{
    let program_str = std::fs::read_to_string(path).expect("");
    
    return program_str.lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.split_whitespace().collect())
                .map(|v| to_instruction(&v))
                .collect();
}
