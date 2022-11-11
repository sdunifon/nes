use std::ops::Index;
use super::op_code::OpCode;

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

pub struct ProgramCounter(u16);


impl From::<ProgramCounter> for usize {
    fn from(pc: ProgramCounter) -> usize {
        pc.0 as usize
    }
}


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
            let opscode  = program[self.program_counter.0 as usize]; // todo convert to opcode
            // let opscode  = program[self.program_counter.into::<usize>()]; // todo convert to opcode
            self.program_counter.next();
            match opscode {
                0x00=> {
                    return;
                },
                0xA9=> {
                    let param = program[self.program_counter.0 as usize];
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
                1_u8..=168_u8 | 170_u8..=u8::MAX => todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        let mut cpu = Cpu::default();
        cpu.interpret(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }

    #[test]
    fn test_cpu_zero_flag() {
        let mut cpu = Cpu::default();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    fn test_cpu_negative_flag() {
        let mut cpu = Cpu::default();
        cpu.interpret(vec![0xA9, 0x80, 0x00]);
        assert_eq!(cpu.register_a, 0x80);
        assert_eq!(cpu.status, 0b1000_0000);
    }
}