use crate::instruction::*;

use crate::input::InputBuffer;
use crate::output::OutputBuffer;

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

    fn peek(&mut self) -> u32{
        return self.stack[self.sp - 1];
    }

    fn store_mem(&mut self, value: u32, address: usize){
        self.memory[address] = value;
    }

    fn fetch_mem(&mut self, address: usize) -> u32{
        return self.memory[address];
    }

    fn execute_instruction<T, G>(&mut self, program: &Program, input: &mut InputBuffer<T>, output: &mut OutputBuffer<G>)
    where std::io::Cursor<T>: std::io::Read,
          G: FnMut(u32, &bounded_vec_deque::BoundedVecDeque<u32>) -> ()
    {
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

            InstId::OUT => {
                output.out(self.pop());
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

            InstId::INC => {
                self.stack[self.sp - 1] = self.stack[self.sp - 1] + 1;
            },

            InstId::DEC => {
                self.stack[self.sp - 1] = self.stack[self.sp - 1] - 1;
            },

            InstId::ADD => {
                let a = self.pop();
                let b = self.peek();
                self.stack[self.sp - 1] = a + b;
            },

            InstId::SUB => {
                let a = self.pop();
                let b = self.peek();
                self.stack[self.sp - 1] = a - b;
            },

            InstId::MUL => {
                let a = self.pop();
                let b = self.peek();
                self.stack[self.sp - 1] = a * b;
            },

            InstId::DIV => {
                let a = self.pop();
                let b = self.peek();
                self.stack[self.sp - 1] = a / b;
            },

            InstId::ADDF => unsafe{
                let a = std::mem::transmute::<u32, f32>(self.pop());
                let b = std::mem::transmute::<u32, f32>(self.peek());
                self.stack[self.sp - 1] = std::mem::transmute::<f32, u32>(a + b);
            },

            InstId::SUBF => unsafe{
                let a = std::mem::transmute::<u32, f32>(self.pop());
                let b = std::mem::transmute::<u32, f32>(self.peek());
                self.stack[self.sp - 1] = std::mem::transmute::<f32, u32>(a - b);
            },

            InstId::MULF => unsafe{
                let a = std::mem::transmute::<u32, f32>(self.pop());
                let b = std::mem::transmute::<u32, f32>(self.peek());
                self.stack[self.sp - 1] = std::mem::transmute::<f32, u32>(a * b);
            },

            InstId::DIVF => unsafe{
                let a = std::mem::transmute::<u32, f32>(self.pop());
                let b = std::mem::transmute::<u32, f32>(self.peek());
                self.stack[self.sp - 1] = std::mem::transmute::<f32, u32>(a / b);
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

            InstId::BREQ => {
                if self.pop() == self.pop(){
                    self.ip[self.cip] += 1;
                }
            }

            InstId::BRNEQ => {
                if self.pop() != self.pop(){
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

            InstId::MOVE => {
                let rel = i.args[0] as usize;
                let value = self.pop();
                self.stack[self.sp - 1 - rel] = value;
            }

            InstId::COPY => {
                let rel = i.args[0] as usize;
                self.stack[self.sp - 1 - rel] = self.peek();
            }

            InstId::POP => {
                self.pop();
            }

            InstId::DEL => {
                let rel = i.args[0] as usize;
                self.stack[(self.sp - 1 - rel)..self.sp].rotate_left(1);
                self.pop();
            }

            InstId::PEEK => {
                let rel = i.args[0] as usize;
                self.push(self.stack[self.sp - 1 - rel]);
            }

            InstId::HALT => {
                self.halted = true;
            }
        }
    }

    #[allow(dead_code)]
    pub fn run<T, G>(&mut self, program: &Program, input: &mut InputBuffer<T>, output: &mut OutputBuffer<G>)
    where std::io::Cursor<T>: std::io::Read,
          G: FnMut(u32, &bounded_vec_deque::BoundedVecDeque<u32>) -> ()
    {
        while !self.halted && self.ip[self.cip] < program.len(){
            self.execute_instruction(&program, input, output);
        }
    }

    #[allow(dead_code)]
    pub fn debug<T, G>(&mut self, program: &Program, input: &mut InputBuffer<T>, output: &mut OutputBuffer<G>, show_lines: bool, show_stats: bool)
    where std::io::Cursor<T>: std::io::Read,
          G: FnMut(u32, &bounded_vec_deque::BoundedVecDeque<u32>) -> ()
    {
        let now = std::time::Instant::now();

        while !self.halted && self.ip[self.cip] < program.len(){
            if show_lines{
                println!("Line {}", self.ip[self.cip]);
            }

            self.execute_instruction(&program, input, output);
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