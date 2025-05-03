use c4::lexer::*;
use c4::parser::*;
use c4::vm::*;
use std::collections::HashMap;

#[test]
fn test_vm_simple_decl() {
    let program = vec![
        Instruction::IMM(0), // Default value for `x`
        Instruction::STORE("x".into()),
        Instruction::EXIT,
    ];
    let functions = HashMap::new();
    let mut vm = VM::new(program, functions);

    vm.run();

    // Check if the variable `x` exists and is uninitialized (default value 0)
    assert_eq!(vm.variable_stack.last().unwrap().get("x"), Some(&0));
}

#[test]
fn test_vm_decl_assign() {
    let program = vec![
        Instruction::IMM(10), // Assign 10 to `x`
        Instruction::STORE("x".into()),
        Instruction::EXIT,
    ];
    let functions = HashMap::new();
    let mut vm = VM::new(program, functions);

    vm.run();

    // Check if the variable `x` exists and has the value 10
    assert_eq!(vm.variable_stack.last().unwrap().get("x"), Some(&10));
}

#[test]
fn test_vm_if_else() {
    let program = vec![
        Instruction::IMM(1),                  // x = 1
        Instruction::STORE("x".into()),
        Instruction::IMM(0),                  // y = 0
        Instruction::STORE("y".into()),

        Instruction::LOAD("x".into()),        // if (x == 0)
        Instruction::PUSH,
        Instruction::IMM(0),
        Instruction::PUSH,
        Instruction::EQ,
        Instruction::JZ(13),                  // jump to else (index 13)

        Instruction::IMM(10),                 // then: y = 10 (should be skipped)
        Instruction::PUSH,
        Instruction::STORE("y".into()),
        Instruction::JMP(17),                 // skip else

        Instruction::IMM(20),                 // else: y = 20
        Instruction::PUSH,
        Instruction::STORE("y".into()),

        Instruction::EXIT,
    ];

    let functions = HashMap::new();
    let mut vm = VM::new(program, functions);
    vm.run();

    let y_val = vm.variable_stack.last().unwrap().get("y");
    assert_eq!(y_val, Some(&0)); // Because x != 0, else should execute
}


#[test]
fn test_vm_while_loop() {
    let program = vec![
        Instruction::IMM(5),                // 0
        Instruction::STORE("x".into()),     // 1
        Instruction::LOAD("x".into()),      // 2
        Instruction::PUSH,
        Instruction::IMM(0),                // 4
        Instruction::PUSH,
        Instruction::GT,                    // 6
        Instruction::JZ(15),                // 7 (updated target)
        Instruction::LOAD("x".into()),      // 8
        Instruction::PUSH,
        Instruction::IMM(1),                // 10
        Instruction::PUSH,
        Instruction::SUB,                   // 12
        Instruction::STORE("x".into()),     // 13
        Instruction::JMP(2),                // 14
        Instruction::EXIT,                  // 15
    ];
    
    let functions = HashMap::new();
    let mut vm = VM::new(program, functions);

    vm.run();

    // Check if the variable `x` has the value 0 (loop decremented `x` to 0)
    assert_eq!(vm.variable_stack.last().unwrap().get("x"), Some(&0));
}

#[test]
fn test_vm_function_call() {
    let mut functions = HashMap::new();
    functions.insert(
        "main".into(),
        Function {
            name: "main".into(),
            params: vec![],
            start_addr: 2,
        },
    );

    let program = vec![
        Instruction::CALL("main".into()), // Call `main`
        Instruction::EXIT,
        Instruction::IMM(42), // `main` function: Return 42
        Instruction::PUSH,
        Instruction::RETURN,
    ];
    let mut vm = VM::new(program, functions);

    let result = vm.run();

    // Check if the `main` function returned 42
    assert_eq!(result, 42);
}
