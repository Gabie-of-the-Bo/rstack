use strum::AsStaticRef;
use byteorder::{BigEndian, WriteBytesExt};
use num_enum::TryFromPrimitive;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone, AsStaticStr, EnumString, TryFromPrimitive)]
pub enum InstId{
    CONST,

    // *** IO ***

    IN32,
    IN16,
    IN8,

    OUT,

    // *** MEMORY ***
    
    STORE,
    STOREC,
    STOREAT,
    FETCH,
    FETCHFROM,

    // *** CASTS ***

    FLOAT,
    CEIL,
    FLOOR,
    ROUND,

    // *** ARITHMETIC ***

    INC,
    DEC,
    ADD,
    SUB,
    MUL,
    DIV,
    ADDF,
    SUBF,
    MULF,
    DIVF,

    // *** COMPLEX OPERATIONS ***

    EXP,
    COS,
    SIN,

    // *** FLOW ***

    JMP,
    BRZ,
    BRNZ,
    BREQ,
    BRNEQ,

    CALL,
    CALLC,
    RET,

    HALT,

    // *** STACK MANIPULATION ***

    PEEK,
    DUP,
    ROTL,
    ROTR,
    COPY,
    MOVE,
    POP,
    DEL
}

impl InstId{
    pub fn arg_number(&self) -> usize{
        match &self{
            InstId::CONST => 1,
            InstId::STOREAT => 1,
            InstId::FETCHFROM => 1,
            InstId::JMP => 1,
            InstId::CALLC => 1,
            InstId::PEEK => 1,
            InstId::ROTL => 1,
            InstId::ROTR => 1,
            InstId::MOVE => 1,
            InstId::COPY => 1,
            InstId::DEL => 1,
            _ => 0
        }
    }
}

pub struct Instruction{
    pub id: InstId,
    pub args: [u32; 2]
}

#[allow(dead_code)]
impl Instruction{
    pub fn to_bytes_u32(&self) -> Vec<u8>{
        let mut res: Vec<u8> = vec!(self.id as u8);

        for arg in 0..self.id.arg_number(){
            res.write_u32::<BigEndian>(self.args[arg] as u32).unwrap();
        }
        
        return res;
    }

    
    pub fn to_bytes_u16(&self) -> Vec<u8>{
        let mut res: Vec<u8> = vec!(self.id as u8);

        for arg in 0..self.id.arg_number(){
            res.write_u16::<BigEndian>(self.args[arg] as u16).unwrap();
        }
        
        return res;
    }

    
    pub fn to_bytes_u8(&self) -> Vec<u8>{
        let mut res: Vec<u8> = vec!(self.id as u8);

        for arg in 0..self.id.arg_number(){
            res.push(self.args[arg] as u8);
        }
        
        return res;
    }
}

pub type Program = Vec<Instruction>;

impl From<InstId> for Instruction{
    fn from(id: InstId) -> Instruction {
        Instruction {
            id: id,
            args: [0, 0]
        }
    }
}

impl From<(InstId, u32)> for Instruction{
    fn from(id: (InstId, u32)) -> Instruction {
        Instruction {
            id: id.0,
            args: [id.1, 0]
        }
    }
}

impl From<(InstId, u32, u32)> for Instruction{
    fn from(id: (InstId, u32, u32)) -> Instruction {
        Instruction {
            id: id.0,
            args: [id.1, id.2]
        }
    }
}

impl std::fmt::Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id.as_static()).expect("Invalid instruction name");

        for arg in 0..self.id.arg_number(){
            write!(f, " {}", self.args[arg]).expect("Invalid instruction argument");
        }

        return std::result::Result::Ok(());
    }
}