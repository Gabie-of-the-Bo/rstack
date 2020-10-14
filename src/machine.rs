use crate::instruction::*;

use crate::input::InputBuffer;

pub struct Machine{
    stack: [u32; 1000],
    memory: [u32; 1000],

    sp: usize,
    ip: [usize; 1000],

    cip: usize,

    halted: bool,
    
    //STATS
    cycles: u32,
    microsecs: u128
}

impl Machine{
    pub fn new() -> Machine{
        return Machine{
            stack: [0; 1000],
            memory: [0; 1000],
            sp: 0,
            ip: [0; 1000],
            cip: 0,
            halted: false,
            cycles: 0,
            microsecs: 0
        };
    }

    fn fetch<'a>(&mut self, program: &'a Program) -> &'a Instruction{
        let res: &Instruction = &program[self.ip[self.cip]];
        self.ip[self.cip] += 1;

        return res;
    }

    fn push(&mut self, value: u32){
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u32{
        self.sp -= 1;
        let res = self.stack[self.sp];

        return res;
    }

    fn store_mem(&mut self, value: u32, address: usize){
        self.memory[address] = value;
    }

    fn fetch_mem(&mut self, address: usize) -> u32{
        return self.memory[address];
    }

    fn execute_instruction<T>(&mut self, program: &Program, input: &mut InputBuffer<T>)
    where std::io::Cursor<T>: std::io::Read{
        let i = self.fetch(&program);

        match i.id{
            InstId::CONST => {
                self.push(i.args[0]);
            },

            InstId::IN32 => {
                self.push(input.read_u32());
            },

            InstId::IN16 => {
                self.push(input.read_u16() as u32);
            },

            InstId::IN8 => {
                self.push(input.read_u8() as u32);
            },

            InstId::STORE => {
                let constant = self.pop();
                let address = self.pop();
                self.store_mem(constant, address as usize);
            },

            InstId::STOREC => {
                let address = self.pop();
                self.store_mem(i.args[0], address as usize);
            },

            InstId::STOREAT => {
                let constant = self.pop();
                self.store_mem(constant, i.args[0] as usize);
            },

            InstId::FETCH => {
                let address = self.pop();
                let value = self.fetch_mem(address as usize);
                self.push(value);
            },

            InstId::FETCHFROM => {
                let value = self.fetch_mem(i.args[0] as usize);
                self.push(value);
            },

            InstId::ADD => {
                let a = self.pop();
                let b = self.pop();
                self.push(a + b);
            },

            InstId::SUB => {
                let a = self.pop();
                let b = self.pop();
                self.push(a - b);
            },

            InstId::MUL => {
                let a = self.pop();
                let b = self.pop();
                self.push(a * b);
            },

            InstId::DIV => {
                let a = self.pop();
                let b = self.pop();
                self.push(a / b);
            },

            InstId::JMP => {
                let line = i.args[0];
                self.ip[self.cip] = line as usize;
            }

            InstId::BRZ => {
                if self.pop() == 0{
                    self.ip[self.cip] += 1;
                }
            }

            InstId::BRNZ => {
                if self.pop() != 0{
                    self.ip[self.cip] += 1;
                }
            }

            InstId::CALL => {
                let line = self.pop();
                self.cip += 1;
                self.ip[self.cip] = line as usize;
            }

            InstId::CALLC => {
                let line = i.args[0];
                self.cip += 1;
                self.ip[self.cip] = line as usize;
            }

            InstId::RET => {
                self.cip -= 1;
            }

            InstId::DUP => {
                self.push(self.stack[self.sp - 1]);
            }

            InstId::ROTL => {
                let rel = i.args[0] as usize;
                self.stack[(self.sp - 1 - rel)..self.sp].rotate_left(1);
            }

            InstId::ROTR => {
                let rel = i.args[0] as usize;
                self.stack[(self.sp - 1 - rel)..self.sp].rotate_right(1);
            }

            InstId::PEEK => {
                let rel = i.args[0] as usize;
                self.push(self.stack[self.sp - 1 - rel]);
            }

            InstId::HALT => {
                self.halted = true;
            }

            InstId::SHOW => println!("{}", self.pop()),
        }
    }

    #[allow(dead_code)]
    pub fn run<T>(&mut self, program: &Program, input: &mut InputBuffer<T>)
    where std::io::Cursor<T>: std::io::Read{
        while !self.halted && self.ip[self.cip] < program.len(){
            self.execute_instruction(&program, input);
        }
    }

    #[allow(dead_code)]
    pub fn debug<T>(&mut self, program: &Program, input: &mut InputBuffer<T>, show_lines: bool, show_stats: bool)
    where std::io::Cursor<T>: std::io::Read{
        let now = std::time::Instant::now();

        while !self.halted && self.ip[self.cip] < program.len(){
            if show_lines{
                println!("Line {}", self.ip[self.cip]);
            }

            self.execute_instruction(&program, input);
            self.cycles += 1;
        }

        self.microsecs += now.elapsed().as_micros();

        if show_stats{
            println!("\n+--------------------------------------+");
            println!("|              STATISTICS              |");
            println!("+--------------------------------------+");
            println!("| * Total executed cycles: {}", self.cycles);
            println!("| * Total elapsed time:    {} μs", self.microsecs);
            println!("+---------------------------------------");
        }
    }
}