use c4::lexer::*;

#[test]
fn test_lexer_basic_numbers_and_ids() {
    let source = "int x = 42;";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Id("x".into()),
        Token::Assign,
        Token::Num(42),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_arithmetic_ops() {
    let source = "a + b - 3 * 4 / 2 % 1;";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Id("a".into()),
        Token::Add,
        Token::Id("b".into()),
        Token::Sub,
        Token::Num(3),
        Token::Mul,
        Token::Num(4),
        Token::Div,
        Token::Num(2),
        Token::Mod,
        Token::Num(1),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_comparisons() {
    let source = "x == y != z < a > b <= c >= d;";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Id("x".into()),
        Token::Eq,
        Token::Id("y".into()),
        Token::Ne,
        Token::Id("z".into()),
        Token::Lt,
        Token::Id("a".into()),
        Token::Gt,
        Token::Id("b".into()),
        Token::Le,
        Token::Id("c".into()),
        Token::Ge,
        Token::Id("d".into()),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_keywords_and_symbols() {
    let source = "if (x) { return 0; } else while (1) {}";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::If,
        Token::LParen,
        Token::Id("x".into()),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::Num(0),
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::While,
        Token::LParen,
        Token::Num(1),
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_simple_decl() {
    let mut lexer = Lexer::new("int x = 100;");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Id("x".into()),
        Token::Assign,
        Token::Num(100),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_arithmetic() {
    let mut lexer = Lexer::new("a + b - 2 * 3 / 4 % 5;");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Id("a".into()),
        Token::Add,
        Token::Id("b".into()),
        Token::Sub,
        Token::Num(2),
        Token::Mul,
        Token::Num(3),
        Token::Div,
        Token::Num(4),
        Token::Mod,
        Token::Num(5),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_keywords() {
    let mut lexer = Lexer::new("if (x) { return 1; } else while (0) {}");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::If,
        Token::LParen,
        Token::Id("x".into()),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::Num(1),
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::While,
        Token::LParen,
        Token::Num(0),
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_string_literal() {
    let mut lexer = Lexer::new("printf(\"Hello, world!\");");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Id("printf".into()),
        Token::LParen,
        Token::Str("Hello, world!".into()),
        Token::RParen,
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_char_literal() {
    let mut lexer = Lexer::new("char c = 'a';");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Char,
        Token::Id("c".into()),
        Token::Assign,
        Token::Num('a' as i64),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_pointer_ops() {
    let mut lexer = Lexer::new("int* p = &x;");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Mul,
        Token::Id("p".into()),
        Token::Assign,
        Token::And,
        Token::Id("x".into()),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_invalid_tokens() {
    let mut lexer = Lexer::new("@ $ # ~");
    let tokens = lexer.tokenize();

    // If your lexer skips unknowns silently:
    let expected = vec![
        Token::Unknown('@'),
        Token::Unknown('$'),
        Token::EOF,
    ];

    // If it logs or errors, adjust this based on your design
    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_with_whitespace() {
    let mut lexer = Lexer::new("  int   x\t=\n42 ;  ");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Id("x".into()),
        Token::Assign,
        Token::Num(42),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize();

    assert_eq!(tokens, vec![Token::EOF]);
}

#[test]
fn test_lexer_mixed_valid_invalid() {
    let mut lexer = Lexer::new("int x = 5; @");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Id("x".into()),
        Token::Assign,
        Token::Num(5),
        Token::Semicolon,
        Token::Unknown('@'),
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_function_def_and_call() {
    let mut lexer = Lexer::new("int sum(int a, int b) { return a + b; } sum(1, 2);");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Id("sum".into()),
        Token::LParen,
        Token::Int,
        Token::Id("a".into()),
        Token::Comma,
        Token::Int,
        Token::Id("b".into()),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::Id("a".into()),
        Token::Add,
        Token::Id("b".into()),
        Token::Semicolon,
        Token::RBrace,
        Token::Id("sum".into()),
        Token::LParen,
        Token::Num(1),
        Token::Comma,
        Token::Num(2),
        Token::RParen,
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_pointers() {
    let mut lexer = Lexer::new("int* ptr = &x; *ptr = 5;");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Mul,
        Token::Id("ptr".into()),
        Token::Assign,
        Token::And,
        Token::Id("x".into()),
        Token::Semicolon,
        Token::Mul,
        Token::Id("ptr".into()),
        Token::Assign,
        Token::Num(5),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_mixed_whitespace() {
    let mut lexer = Lexer::new("int\tmain  (  ) \n { return\t0 ; } ");
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Int,
        Token::Id("main".into()),
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::Num(0),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}
