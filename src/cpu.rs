use std::ops::Index;

pub struct Cpu {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: ProgramCounter,
}

impl Default for Cpu{
    fn default() -> Self {
        Self{
            register_a: 0,
            status: 0,
            program_counter: ProgramCounter(0),
        }

    }
}

struct ProgramCounter(u16);

struct Program {
    data: Vec<OpCode>
}

impl Index<ProgramCounter> for Program {
    type Output = OpCode;

    fn index(&self, index: ProgramCounter) -> &Self::Output {
        &self.data[index.0 as usize]
    }
}
impl ProgramCounter{
    fn next(&mut self){
        self.0 += 1
    }
}



impl Cpu {
    pub fn interpret(&mut self, program: Vec<u8>){
        self.program_counter = ProgramCounter(0);

        loop {
            let opscode: OpCode = program[self.program_counter as usize]; // todo convert to opcode
            self.program_counter.next();
            match opscode {
                OpCode::_0xA9 => {
                    let param = program[self.program_counter];
                    self.program_counter.next();
                    self.register_a = param;

                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    }else {
                        self.status = self.status & 0b1111_1101;
                    }
                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    }else {
                        self.status = self.status & 0b0111_1111;
                    }

                }
            }
        }
    }
}

enum OpCode {
    _0xA9

}