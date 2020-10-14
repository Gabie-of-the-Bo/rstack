#[allow(dead_code)]
#[derive(Copy, Clone)]
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
        write!(f, "{} {:?}", self.id as u8, self.args)
    }
}