// Virtual Machine file
use crate::parser::ASTNode; // used to convert ast to instructions
use crate::lexer::Token;    // our token enum
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub enum Instruction { // instruction types
    IMM(i32),
    LC, LI, SC, SI,
    PUSH,
    JMP(usize), JZ(usize), JNZ(usize),
    CALL(usize),
    ENT(usize),
    ADJ(usize),
    LEV, LEA(usize),
    LOAD(String), STORE(String), 
    PRINTF(String, Vec<String>),
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
    pub variables: HashMap<String, i32>,
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
            variables: HashMap::new(),
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
                Instruction::STORE(name) => self.exec_store(name.as_str()),
                Instruction::LOAD(name) => self.exec_load(name),
                Instruction::PRINTF(fmt, args) => self.exec_printf(&fmt, &args),
                Instruction::EXIT => return self.ax,
                _ => panic!("Unsupported instruction: {:?}", self.text[self.pc - 1]),
            }
        }
    }

    fn fetch(&mut self) -> Instruction {
        let op = self.text[self.pc].clone();
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

    fn exec_store(&mut self, name: &str) {
        self.variables.insert(name.to_string(), self.ax);
    }
    
    fn exec_load(&mut self, name: String) {
        self.ax = *self.variables.get(&name).unwrap_or(&0);
    }

    pub fn exec_printf(&self, fmt: &String, args: &Vec<String>) {
        let mut output = fmt.clone();

        for var in args {
            if let Some(val) = self.variables.get(var) {
                output = output.replacen("%d", &val.to_string(), 1);
            } else {
                panic!("Undefined variable: {}", var);
            }
        }

        if output.contains("%d") {
            panic!("Mismatch between format string and arguments");
        }

        println!("{}", output);
    }
}

pub fn generate(program: Vec<ASTNode>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for node in program {
        generate_node(&node, &mut instructions);
    }
    instructions
}

pub fn generate_node(node: &ASTNode, instructions: &mut Vec<Instruction>) {
    match node {
        ASTNode::Num(value) => {
            instructions.push(Instruction::IMM(*value as i32));
            instructions.push(Instruction::PUSH);
        }
        ASTNode::Id(name) => {
            instructions.push(Instruction::LOAD(name.clone()));
            instructions.push(Instruction::PUSH);
        }
        ASTNode::Assign { name, value } => {
            generate_node(value, instructions);
            instructions.push(Instruction::STORE(name.clone()));
        }
        ASTNode::BinaryOp { op, left, right } => {
            generate_node(left, instructions);
            generate_node(right, instructions);
            match op {
                Token::Add => instructions.push(Instruction::ADD),
                Token::Sub => instructions.push(Instruction::SUB),
                Token::Mul => instructions.push(Instruction::MUL),
                Token::Div => instructions.push(Instruction::DIV),
                Token::Mod => instructions.push(Instruction::MOD),
                _ => panic!("Unsupported binary operator {:?}", op),
            }
        }
        ASTNode::FuncCall { name, args } => {
            if name == "printf" {
                if args.is_empty() {
                    panic!("printf requires at least a format string");
                }

                match &args[0] {
                    ASTNode::Str(message) => {
                        let mut fmt_args = Vec::new();
                        for arg in &args[1..] {
                            if let ASTNode::Id(var_name) = arg {
                                fmt_args.push(var_name.clone());
                            } else {
                                panic!("Only variable identifiers allowed as printf arguments");
                            }
                        }
                        instructions.push(Instruction::PRINTF(message.clone(), fmt_args));
                    }
                    _ => panic!("printf must start with a string literal"),
                }
            } else {
                panic!("Function call '{}' not supported yet", name);
            }
        }
        ASTNode::FuncDef { name, body, .. } => {
            if name == "main" {
                for stmt in body {
                    generate_node(stmt, instructions);
                }
                instructions.push(Instruction::EXIT);
            } else {
                panic!("Only 'main' function is supported for now.");
            }
        }
        _ => panic!("Unsupported AST node {:?}", node),
    }
}