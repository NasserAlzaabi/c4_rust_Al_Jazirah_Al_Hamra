// Virtual Machine file

#[derive(Debug, Clone, Copy)]
pub enum Instruction { // instruction types
    IMM(i32),
    LC, LI, SC, SI,
    PUSH,
    JMP(usize), JZ(usize), JNZ(usize),
    CALL(usize),
    ENT(usize),
    ADJ(usize),
    LEV, LEA(usize),
    OR, XOR, AND, EQ, NE, LT, LE, GT, GE, SHL, SHR,
    ADD, SUB, MUL, DIV, MOD,
    EXIT,
}

pub struct VM {
    pub text: Vec<Instruction>,
    pub stack: Vec<i32>,
    pub pc: usize,
    pub bp: usize,
    pub sp: usize,
    pub ax: i32,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            text: program,
            stack: vec![0; 10000],
            pc: 0,
            bp: 0,
            sp: 0,
            ax: 0,
        }
    }

    pub fn run(&mut self) -> i32 {
        loop {
            match self.fetch() {
                Instruction::IMM(val) => self.exec_imm(val),
                Instruction::PUSH => self.exec_push(),
                Instruction::ADD => self.exec_add(),
                Instruction::SUB => self.exec_sub(),
                Instruction::MUL => self.exec_mul(),
                Instruction::DIV => self.exec_div(),
                Instruction::MOD => self.exec_mod(),
                Instruction::EXIT => return self.ax,
                _ => panic!("Unimplemented instruction: {:?}", self.text[self.pc - 1]),
            }
        }
    }

    fn fetch(&mut self) -> Instruction {
        let op = self.text[self.pc];
        self.pc += 1;
        op
    }

    fn exec_imm(&mut self, val: i32) {
        self.ax = val;
    }

    fn exec_push(&mut self) {
        self.stack[self.sp] = self.ax;
        self.sp += 1;
    }

    fn exec_add(&mut self) {
        self.sp -= 1;
        self.ax = self.stack[self.sp] + self.ax;
    }

    fn exec_sub(&mut self) {
        self.sp -= 1;
        self.ax = self.stack[self.sp] - self.ax;
    }

    fn exec_mul(&mut self) {
        self.sp -= 1;
        self.ax = self.stack[self.sp] * self.ax;
    }

    fn exec_div(&mut self) {
        self.sp -= 1;
        self.ax = self.stack[self.sp] / self.ax;
    }

    fn exec_mod(&mut self) {
        self.sp -= 1;
        self.ax = self.stack[self.sp] % self.ax;
    }
}
