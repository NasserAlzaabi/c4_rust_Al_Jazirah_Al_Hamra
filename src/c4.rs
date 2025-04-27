// Main file

mod vm;
use vm::{Instruction, VM};

fn main() {
    let program = vec![
        Instruction::IMM(10),
        Instruction::PUSH,
        Instruction::IMM(5),
        Instruction::ADD,
        Instruction::EXIT,
    ];

    let mut vm = VM::new(program);
    let result = vm.run();
    println!("Result: {}", result); // should print 15
}