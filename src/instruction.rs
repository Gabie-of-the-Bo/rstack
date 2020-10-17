use strum::AsStaticRef;

#[allow(dead_code)]
#[derive(Copy, Clone, AsStaticStr, EnumString)]
pub enum InstId{
    CONST,

    IN32,
    IN16,
    IN8,

    OUT,

    STORE,
    STOREC,
    STOREAT,
    FETCH,
    FETCHFROM,

    ADD,
    SUB,
    MUL,
    DIV,
    ADDF,
    SUBF,
    MULF,
    DIVF,

    JMP,
    BRZ,
    BRNZ,

    CALL,
    CALLC,
    RET,

    PEEK,
    DUP,
    ROTL,
    ROTR,

    HALT
}

impl InstId{
    fn arg_number(&self) -> usize{
        match &self{
            InstId::CONST => 1,
            InstId::STOREAT => 1,
            InstId::FETCHFROM => 1,
            InstId::JMP => 1,
            InstId::CALLC => 1,
            InstId::ROTL => 1,
            InstId::ROTR => 1,
            _ => 0
        }
    }
}

pub struct Instruction{
    pub id: InstId,
    pub args: [u32; 2]
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
        write!(f, "{} {:?}", self.id.as_static(), self.args)
    }
}