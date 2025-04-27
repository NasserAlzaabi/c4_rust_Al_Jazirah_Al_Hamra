use c4::vm::{Instruction, VM}; // Import VM and Instruction via package name

#[test]
fn test_addition() {
    let program = vec![ //example instructions
        Instruction::IMM(2),
        Instruction::PUSH,
        Instruction::IMM(3),
        Instruction::ADD,
        Instruction::EXIT,
    ];
    let mut vm = VM::new(program);
    let result = vm.run();
    assert_eq!(result, 5); //checks if the result is the same as expected output
}

#[test]
fn test_subtraction() {
    let program = vec![
        Instruction::IMM(10),
        Instruction::PUSH,
        Instruction::IMM(4),
        Instruction::SUB,
        Instruction::EXIT,
    ];
    let mut vm = VM::new(program);
    let result = vm.run();
    assert_eq!(result, 6);
}

#[test]
fn test_multiplication() {
    let program = vec![
        Instruction::IMM(7),
        Instruction::PUSH,
        Instruction::IMM(6),
        Instruction::MUL,
        Instruction::EXIT,
    ];
    let mut vm = VM::new(program);
    let result = vm.run();
    assert_eq!(result, 42);
}

#[test]
fn test_division() {
    let program = vec![
        Instruction::IMM(20),
        Instruction::PUSH,
        Instruction::IMM(5),
        Instruction::DIV,
        Instruction::EXIT,
    ];
    let mut vm = VM::new(program);
    let result = vm.run();
    assert_eq!(result, 4);
}

#[test]
fn test_modulus() {
    let program = vec![
        Instruction::IMM(20),
        Instruction::PUSH,
        Instruction::IMM(6),
        Instruction::MOD,
        Instruction::EXIT,
    ];
    let mut vm = VM::new(program);
    let result = vm.run();
    assert_eq!(result, 2);
}
