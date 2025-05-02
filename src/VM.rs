// Virtual Machine file
use crate::parser::ASTNode; // used to convert ast to instructions
use crate::lexer::Token;    // our token enum
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    IMM(i32),
    LC, LI, SC, SI,
    PUSH,
    JMP(usize), JZ(usize), JNZ(usize),
    CALL(String),
    RETURN,
    ENT(usize),
    ADJ(usize),
    LEV, LEA(usize),
    LOAD(String), STORE(String),
    PRINTF(String, Vec<String>),
    OR, XOR, AND, EQ, NE, LT, LE, GT, GE, SHL, SHR,
    ADD, SUB, MUL, DIV, MOD,
    EXIT,
}

pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub start_addr: usize,
}

pub struct VM {
    pub text: Vec<Instruction>,
    pub stack: Vec<i32>,
    pub pc: usize,
    pub bp: usize,
    pub sp: usize,
    pub ax: i32,
    pub variables: HashMap<String, i32>,
    pub functions: HashMap<String, Function>,
    pub call_stack: Vec<usize>,
}

impl VM {
    pub fn new(program: Vec<Instruction>, functions: HashMap<String, Function>) -> Self {
        Self {
            text: program,
            stack: vec![0; 10000],
            pc: 0,
            bp: 0,
            sp: 0,
            ax: 0,
            variables: HashMap::new(),
            functions,
            call_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> i32 {
        while self.pc < self.text.len() {
            self.execute_instruction();
        }
        self.ax
    }

    fn execute_instruction(&mut self) {
        match self.fetch() {
            Instruction::IMM(val) => self.exec_imm(val),
            Instruction::PUSH => self.exec_push(),
            Instruction::ADD => self.exec_add(),
            Instruction::SUB => self.exec_sub(),
            Instruction::MUL => self.exec_mul(),
            Instruction::DIV => self.exec_div(),
            Instruction::MOD => self.exec_mod(),
            Instruction::GT => self.exec_gt(),
            Instruction::LT => self.exec_lt(),
            Instruction::EQ => self.exec_eq(),
            Instruction::NE => self.exec_ne(),
            Instruction::JZ(addr) => self.exec_jz(addr),
            Instruction::JMP(addr) => self.exec_jmp(addr),
            Instruction::CALL(name) => self.exec_call(&name),
            Instruction::RETURN => self.exec_return(),
            Instruction::STORE(name) => self.exec_store(&name),
            Instruction::LOAD(name) => self.exec_load(name),
            Instruction::PRINTF(fmt, args) => self.exec_printf(&fmt, &args),
            Instruction::EXIT => self.pc = self.text.len(),
            _ => panic!("Unsupported instruction: {:?}", self.text[self.pc - 1]),
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
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = left + right;
    }

    fn exec_sub(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = left - right;
    }

    fn exec_mul(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = left * right;
    }

    fn exec_div(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = left / right;
    }

    fn exec_mod(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = left % right;
    }

    fn exec_gt(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = (left > right) as i32;
    }

    fn exec_lt(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = (left < right) as i32;
    }

    fn exec_eq(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = (left == right) as i32;
    }

    fn exec_ne(&mut self) {
        self.sp -= 1;
        let right = self.stack[self.sp];
        self.sp -= 1;
        let left = self.stack[self.sp];
        self.ax = (left != right) as i32;
    }

    fn exec_jz(&mut self, addr: usize) {
        if self.ax == 0 {
            self.pc = addr;
        }
    }

    fn exec_jmp(&mut self, addr: usize) {
        self.pc = addr;
    }

    fn exec_call(&mut self, name: &str) {
        if let Some(func) = self.functions.get(name) {
            println!(
                "CALL: sp = {}, ax = {}, pc = {}, calling {}",
                self.sp, self.ax, self.pc, name
            );
            self.call_stack.push(self.pc); // Save the return address
            self.pc = func.start_addr; // Jump to the function
        } else {
            panic!("Undefined function: {}", name);
        }
    }    

    fn exec_return(&mut self) {
        println!(
            "RETURN: sp = {}, ax = {}, call_stack = {:?}",
            self.sp, self.ax, self.call_stack
        );
    
        if let Some(return_addr) = self.call_stack.pop() {
            if self.sp == 0 {
                self.ax = 0; // Default return value
            } else {
                self.ax = self.stack[self.sp - 1]; // Pop return value into ax
                self.sp -= 1; // Adjust stack pointer
            }
            self.pc = return_addr; // Restore the program counter
        } else {
            // Terminate if the call stack is empty (main is returning)
            println!("Main function returned, terminating program with result: {}", self.ax);
            self.pc = self.text.len(); // Terminate program
        }
    }        

    fn exec_store(&mut self, name: &str) {
        println!("Storing variable: {} = {}", name, self.ax);
        self.variables.insert(name.to_string(), self.ax);
    }    

    fn exec_load(&mut self, name: String) {
        self.ax = *self.variables.get(&name).unwrap_or(&0);
    }
    
    pub fn exec_printf(&mut self, fmt: &String, args: &Vec<String>) {
        println!("Executing PRINTF: fmt = {:?}, args = {:?}", fmt, args);
    
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

pub fn generate(program: Vec<ASTNode>) -> (Vec<Instruction>, HashMap<String, Function>) {
    let mut instructions = vec![Instruction::CALL("main".to_string())];
    let mut functions = HashMap::new();
    let mut func_defs = Vec::new();

    // Separate function definitions and top-level expressions
    for node in program {
        if let ASTNode::FuncDef { .. } = node {
            func_defs.push(node);
        } else {
            generate_node_with_push(&node, &mut instructions, true);
        }
    }

    // Generate function definitions after the call
    for node in func_defs {
        if let ASTNode::FuncDef { name, params, body, .. } = node {
            let start_addr = instructions.len();
            for stmt in body {
                generate_node_with_push(&stmt, &mut instructions, false);
            }

            // Ensure there is a return instruction
            if instructions.last() != Some(&Instruction::RETURN) {
                instructions.push(Instruction::IMM(0)); // default return value
                instructions.push(Instruction::RETURN);
            }

            let param_names: Vec<String> = params.iter().map(|(_, name)| name.clone()).collect();
            functions.insert(
                name.clone(),
                Function {
                    name: name.clone(),
                    params: param_names,
                    start_addr,
                },
            );
        }
    }

    instructions.push(Instruction::EXIT);
    (instructions, functions)
}



fn generate_node_with_push(node: &ASTNode, instructions: &mut Vec<Instruction>, push_result: bool) {
    match node {
        ASTNode::Num(value) => {
            instructions.push(Instruction::IMM(*value as i32));
            if push_result {
                instructions.push(Instruction::PUSH);
            }
        }
        ASTNode::Id(name) => {
            instructions.push(Instruction::LOAD(name.clone()));
            if push_result {
                instructions.push(Instruction::PUSH);
            }
        }
        ASTNode::Assign { name, value } => {
            generate_node_with_push(value, instructions, true);
            instructions.push(Instruction::STORE(name.clone()));
        }
        ASTNode::BinaryOp { op, left, right } => {
            generate_node_with_push(left, instructions, true);
            generate_node_with_push(right, instructions, true);
            match op {
                Token::Add => instructions.push(Instruction::ADD),
                Token::Sub => instructions.push(Instruction::SUB),
                Token::Mul => instructions.push(Instruction::MUL),
                Token::Div => instructions.push(Instruction::DIV),
                Token::Mod => instructions.push(Instruction::MOD),
                Token::Gt => instructions.push(Instruction::GT),
                Token::Lt => instructions.push(Instruction::LT),
                Token::Eq => instructions.push(Instruction::EQ),
                Token::Ne => instructions.push(Instruction::NE),
                _ => panic!("Unsupported binary operator {:?}", op),
            }
            if push_result {
                instructions.push(Instruction::PUSH);
            }
        }
        ASTNode::If { cond, then_branch, else_branch } => {
            generate_node_with_push(cond, instructions, false);
            instructions.push(Instruction::JZ(0)); // placeholder
            let jz_index = instructions.len() - 1;

            generate_node_with_push(then_branch, instructions, false);

            if let Some(else_branch) = else_branch {
                instructions.push(Instruction::JMP(0)); // placeholder
                let jmp_index = instructions.len() - 1;

                let else_start = instructions.len();
                generate_node_with_push(else_branch, instructions, false);
                instructions[jz_index] = Instruction::JZ(else_start);
                instructions[jmp_index] = Instruction::JMP(instructions.len());
            } else {
                instructions[jz_index] = Instruction::JZ(instructions.len());
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
                        println!("Generating PRINTF: fmt = {:?}, args = {:?}", message, fmt_args);
                        instructions.push(Instruction::PRINTF(message.clone(), fmt_args));
                    }
                    _ => panic!("printf must start with a string literal"),
                }
            } else {
                for arg in args {
                    generate_node_with_push(arg, instructions, true);
                }
                instructions.push(Instruction::CALL(name.clone()));
                if push_result {
                    instructions.push(Instruction::PUSH);
                }                
            }
        }               
        ASTNode::Return(expr) => {
            generate_node_with_push(expr, instructions, true); // Generate code for the return value
            instructions.push(Instruction::RETURN); // Emit the RETURN instruction
        }        
        // ASTNode::FuncDef { name, body, .. } => {
        //     let start_addr = instructions.len();
        
        //     for stmt in body {
        //         generate_node_with_push(stmt, instructions, false);
        //     }
        
        //     // Push default return value if no explicit return exists
        //     if instructions.last() != Some(&Instruction::RETURN) {
        //         instructions.push(Instruction::IMM(0));
        //         instructions.push(Instruction::RETURN);
        //     }
        // }
        _ => panic!("Unsupported AST node {:?}", node),
    }
}
