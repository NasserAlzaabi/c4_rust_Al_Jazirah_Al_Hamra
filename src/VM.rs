// Virtual Machine file
use crate::parser::ASTNode; // used to convert ast to instructions
use crate::lexer::Token;    // our token enum
use std::collections::HashMap;

/// Represents instructions that can be executed by the VM
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]  // Suppress warnings for unused variants
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
    LoadString(String),
    DEREF,
    ADDR(String),
}

/// Represents a function definition with parameters and entry point
pub struct Function {
    #[allow(dead_code)]  // Suppress warning for unused field
    pub name: String,
    pub params: Vec<String>,
    pub start_addr: usize,
}

/// Virtual machine that executes compiled instructions
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
    pub variable_stack: Vec<HashMap<String, i32>>,
}

impl VM {
    /// Creates a new VM with given program instructions and function definitions
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
            variable_stack: vec![HashMap::new()],
        }
    }

    /// Runs the program from start to finish and returns the final result
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
            Instruction::ENT(size) => self.exec_ent(size),
            Instruction::ADJ(size) => self.exec_adj(size),
            Instruction::LEV => self.exec_lev(),
            Instruction::LEA(offset) => self.exec_lea(offset),
            Instruction::CALL(name) => self.exec_call(&name),
            Instruction::RETURN => self.exec_return(),
            Instruction::STORE(name) => self.exec_store(&name),
            Instruction::LOAD(name) => self.exec_load(name),
            Instruction::PRINTF(fmt, args) => self.exec_printf(&fmt, &args),
            Instruction::EXIT => self.pc = self.text.len(),
            Instruction::LoadString(string) => self.exec_load_string(string),  // Fixed: lowercase variable name
            Instruction::DEREF => self.exec_deref(),
            Instruction::ADDR(name) => self.exec_addr(&name),
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

    fn exec_ent(&mut self, size: usize) {
        // Save old base pointer
        self.stack[self.sp] = self.bp as i32;
        self.sp += 1;
        
        // Set new base pointer to current stack pointer
        self.bp = self.sp;
        
        // Allocate stack space for local variables
        self.sp += size;
    }

    fn exec_adj(&mut self, size: usize) {
        // Adjust stack pointer (typically used after pushing function arguments)
        self.sp += size;
    }

    fn exec_lev(&mut self) {
        // Leave function - restore stack pointer and base pointer
        self.sp = self.bp;
        
        // Restore previous base pointer
        self.sp -= 1;
        self.bp = self.stack[self.sp] as usize;
        
        // Fetch return address
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }

    fn exec_lea(&mut self, offset: usize) {
        // Load effective address (local variable relative to bp)
        self.ax = (self.bp + offset) as i32;
    }

    fn exec_call(&mut self, name: &str) {
        if let Some(func) = self.functions.get(name) {
            self.call_stack.push(self.pc); // Save the return address
            self.pc = func.start_addr; // Jump to the function
    
            // Handle function parameters
            let param_count = func.params.len();
            let mut param_values = Vec::new();
            for _ in 0..param_count {
                self.sp -= 1;
                param_values.push(self.stack[self.sp]);
            }
            param_values.reverse(); // Reverse to maintain the correct order
    
            // Create a new scope for the function's variables
            let mut local_vars = HashMap::new();
            for (param_name, value) in func.params.iter().zip(param_values.iter()) {
                local_vars.insert(param_name.clone(), *value);
            }
            self.variable_stack.push(local_vars);
        } else {
            panic!("Undefined function: {}. Available functions: {:?}", 
                  name, self.functions.keys().collect::<Vec<_>>());
        }
    }  

    fn exec_return(&mut self) {
        self.variable_stack.pop();

        if let Some(return_addr) = self.call_stack.pop() {
            if self.sp > 0 {
                self.sp -= 1;
                self.ax = self.stack[self.sp];
            }
            self.pc = return_addr; // Restore the program counter
        } else {
            // Terminate if the call stack is empty (main is returning)
            self.pc = self.text.len(); // Terminate program
        }
    }        

    fn exec_store(&mut self, name: &str) {
        if let Some(scope) = self.variable_stack.last_mut() {
            scope.insert(name.to_string(), self.ax);
        } else {
            panic!("No variable scope found");
        }
    }
    
    fn exec_load(&mut self, name: String) {
        for scope in self.variable_stack.iter().rev() {
            if let Some(value) = scope.get(&name) {
                self.ax = *value;
                return;
            }
        }
        panic!("Undefined variable: {}", name);
    }
    
    /// Executes a printf instruction with format string and arguments
    pub fn exec_printf(&mut self, fmt: &String, args: &Vec<String>) {
        let mut output = fmt.clone();

        for _ in args {
            if self.sp == 0 {
                panic!("Not enough values on the stack for printf");
            }
            self.sp -= 1;
            let val = self.stack[self.sp];
            output = output.replacen("%d", &val.to_string(), 1);
        }
        if output.contains("%d") {
            panic!("Mismatch between format string and arguments");
        }
        println!("{}", output);
    }    

    fn exec_load_string(&mut self, string: String) {
        // Simulate storing the string in memory and returning its address
        let address = self.stack.len(); // Use the stack length as a fake address
        self.variables.insert(string.clone(), address as i32);
        self.ax = address as i32;
    }

    fn exec_deref(&mut self) {
        // Dereference a pointer - in our simplified model, this would
        // fetch the value at the "address" stored in ax
        let ptr = self.ax as usize;
        if ptr < self.stack.len() {
            self.ax = self.stack[ptr];
        } else {
            // For string pointers, just return the address itself
            // This simplification works for printf("%d\n", s) which just prints the address
        }
    }

    fn exec_addr(&mut self, name: &str) {
        // Get the "address" of a variable (simplification)
        for (i, scope) in self.variable_stack.iter().enumerate().rev() {
            if scope.contains_key(name) {
                // Create a fake address based on scope and name
                let fake_address = ((i + 1) * 1000) + name.len() * 10;
                self.ax = fake_address as i32;
                return;
            }
        }
        
        // Check global variables too
        if let Some(val) = self.variables.get(name) {
            self.ax = *val;
            return;
        }
        
        panic!("Variable not found: {}", name);
    }
}

/// Converts AST nodes into VM instructions and function definitions
/// # Argument: program - The AST nodes representing the program
/// Returns: A tuple containing the instructions and function definitions
pub fn generate(program: Vec<ASTNode>) -> (Vec<Instruction>, HashMap<String, Function>) {
    let mut instructions = vec![
        Instruction::CALL("main".to_string()),
        Instruction::EXIT, // ← make sure EXIT happens AFTER main returns
    ];
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
            
            // Create a new variable scope for the function
            instructions.push(Instruction::ENT(0)); // Will update with local variable count
            
            for stmt in &body {
                generate_node_with_push(&stmt, &mut instructions, false);
            }

            // Ensure there is a return instruction
            if instructions.last() != Some(&Instruction::RETURN) {
                instructions.push(Instruction::IMM(0)); // default return value
                instructions.push(Instruction::PUSH);
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
    
    // Check if main exists before trying to call it
    if !functions.contains_key("main") {
        panic!("No 'main' function found in the program. Check your test.c file.");
    }

    (instructions, functions)
}

/// Generates VM instructions for an AST node and pushes the result if needed
/// 
/// # Arguments
/// node - The AST node to generate instructions for
/// instructions - The vector to append instructions to
/// push_result - Whether to push the result onto the stack
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
        ASTNode::Str(_message) => {
            if push_result {
                panic!("Unexpected string literal used as an expression");
            }
        }
        ASTNode::DeclAssign { typename, name, value } => {
            match typename {
                Token::Char => {
                    generate_node_with_push(value, instructions, true);
                    instructions.push(Instruction::STORE(name.clone()));
                }
                Token::CharPointer => {
                    if let ASTNode::Str(string) = &**value {
                        instructions.push(Instruction::LoadString(string.clone()));
                        instructions.push(Instruction::STORE(name.clone()));
                    } else {
                        panic!("Invalid value for char*");
                    }
                }
                _ => {
                    generate_node_with_push(value, instructions, true);
                    instructions.push(Instruction::STORE(name.clone()));
                }
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
                        for (i, arg) in args.iter().enumerate().skip(1).rev() {
                            let arg_name = format!("__printf_arg_{}", i);
                            generate_node_with_push(arg, instructions, true);
                            // Store in a pseudo-variable slot (not actually used by VM logic — it's symbolic)
                            fmt_args.push(arg_name);
                        }
                        instructions.push(Instruction::PRINTF(message.clone(), fmt_args));
                    }
                    _ => panic!("printf must start with a string literal"),
                }
            } else if name == "__block" {
                // Handle special __block function
                for arg in args {
                    generate_node_with_push(arg, instructions, false);
                }
            } else if name == "return" {
                // Handle return statements
                if !args.is_empty() {
                    generate_node_with_push(&args[0], instructions, true);
                } else {
                    instructions.push(Instruction::IMM(0));
                    instructions.push(Instruction::PUSH);
                }
                instructions.push(Instruction::RETURN);
            } else {
                // Regular function calls
                for arg in args.iter().rev() {
                    // Special handling for pointer arguments
                    match arg {
                        ASTNode::Id(var_name) => {
                            // For variable arguments, we use LOAD
                            instructions.push(Instruction::LOAD(var_name.clone()));
                            instructions.push(Instruction::PUSH);
                        },
                        ASTNode::UnaryOp { op: Token::Mul, expr } => {
                            // For dereferenced pointer arguments
                            generate_node_with_push(expr, instructions, true);
                            instructions.push(Instruction::DEREF);
                            instructions.push(Instruction::PUSH);
                        },
                        _ => {
                            // Default handling for other argument types
                            generate_node_with_push(arg, instructions, true);
                        }
                    }
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
        ASTNode::WhileLoop { condition, body } => {
            let loop_start = instructions.len();

            generate_node_with_push(condition, instructions, false); // Evaluate condition
            instructions.push(Instruction::JZ(0)); // Jump to after loop if false
            let jz_index = instructions.len() - 1;

            for stmt in body.iter() {
                generate_node_with_push(stmt, instructions, false);
            }

            instructions.push(Instruction::JMP(loop_start)); // Jump back to start
            let loop_end = instructions.len();

            // Patch JZ with loop_end
            if let Instruction::JZ(ref mut target) = instructions[jz_index] {
                *target = loop_end;
            }
        }        
        // ASTNode::FuncCall { name, args } if name == "__block" => {
        //     for arg in args {
        //         generate_node_with_push(arg, instructions, false);
        //     }
        // },
        ASTNode::Decl { typename, name } => {
            // Default initialize variables
            match typename {
                Token::Int => {
                    instructions.push(Instruction::IMM(0)); // Default value for `int`
                    instructions.push(Instruction::STORE(name.clone()));
                },
                Token::Char => {
                    instructions.push(Instruction::IMM(0)); // Default value for `char`
                    instructions.push(Instruction::STORE(name.clone()));
                }
                Token::CharPointer => {
                    instructions.push(Instruction::IMM(0)); // Default value for `char*` (null pointer)
                    instructions.push(Instruction::STORE(name.clone()));
                }
                _ => panic!("Unsupported type in declaration: {:?}", typename),
            }
        },
        ASTNode::Block(statements) => {
            for stmt in statements {
                generate_node_with_push(stmt, instructions, false);
            }
        },
        ASTNode::UnaryOp { op, expr } => {
            match op {
                Token::Mul => {
                    // Handle pointer dereference
                    generate_node_with_push(expr, instructions, true);
                    instructions.push(Instruction::DEREF);
                    if push_result {
                        instructions.push(Instruction::PUSH);
                    }
                },
                Token::And => {
                    // Handle address-of operator
                    if let ASTNode::Id(name) = &**expr {
                        instructions.push(Instruction::ADDR(name.clone()));
                        if push_result {
                            instructions.push(Instruction::PUSH);
                        }
                    } else {
                        panic!("Address-of operator can only be applied to variables");
                    }
                },
                Token::Not => {
                    // Handle logical NOT
                    generate_node_with_push(expr, instructions, true);
                    instructions.push(Instruction::IMM(0));
                    instructions.push(Instruction::PUSH);
                    instructions.push(Instruction::EQ);
                    if push_result {
                        instructions.push(Instruction::PUSH);
                    }
                },
                // Handle other unary operators
                Token::Inc => {
                    if let ASTNode::Id(name) = &**expr {
                        instructions.push(Instruction::LOAD(name.clone()));
                        instructions.push(Instruction::PUSH);
                        instructions.push(Instruction::IMM(1));
                        instructions.push(Instruction::PUSH);
                        instructions.push(Instruction::ADD);
                        instructions.push(Instruction::STORE(name.clone()));
                    } else {
                        panic!("Increment operator must be applied to a variable");
                    }
                },
                Token::Dec => {
                    if let ASTNode::Id(name) = &**expr {
                        instructions.push(Instruction::LOAD(name.clone()));
                        instructions.push(Instruction::PUSH);
                        instructions.push(Instruction::IMM(1));
                        instructions.push(Instruction::PUSH);
                        instructions.push(Instruction::SUB);
                        instructions.push(Instruction::STORE(name.clone()));
                    } else {
                        panic!("Decrement operator must be applied to a variable");
                    }
                },
                _ => panic!("Unsupported unary operator {:?}", op),
            }
        },
        _ => panic!("Unsupported AST node {:?}", node),
    }
}
