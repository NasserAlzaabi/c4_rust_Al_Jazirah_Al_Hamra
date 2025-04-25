use c4::parser::*;

#[test]
fn test_parse_number() {
    let tokens = vec![Token::Num(42)];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_primary();
    assert_eq!(ast, Some(ASTNode::Num(42)));
}

#[test]
fn test_parse_identifier() {
    let tokens = vec![Token::Id("x".to_string())];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_primary();
    assert_eq!(ast, Some(ASTNode::Id("x".to_string())));
}

#[test]
fn test_parse_parenthesized_number() {
    let tokens = vec![
        Token::LParen,
        Token::Num(123),
        Token::RParen,
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_primary();
    assert_eq!(ast, Some(ASTNode::Num(123)));
}

#[test]
fn test_parse_simple_function_call() {
    let tokens = vec![
        Token::Id("foo".to_string()),
        Token::LParen,
        Token::Num(1),
        Token::Comma,
        Token::Num(2),
        Token::RParen,
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_primary();
    assert_eq!(
        ast,
        Some(ASTNode::FuncCall {
            name: "foo".to_string(),
            args: vec![
                ASTNode::Num(1),
                ASTNode::Num(2),
            ]
        })
    );
}

#[test]
fn test_parse_unary_minus_number() {
    let tokens = vec![
        Token::Sub,
        Token::Num(5),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_unary();
    assert_eq!(
        ast,
        Some(ASTNode::UnaryOp {
            op: Token::Sub,
            expr: Box::new(ASTNode::Num(5)),
        })
    );
}

#[test]
fn test_parse_unary_dereference() {
    let tokens = vec![
        Token::Mul,
        Token::Id("ptr".to_string()),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_unary();
    assert_eq!(
        ast,
        Some(ASTNode::UnaryOp {
            op: Token::Mul,
            expr: Box::new(ASTNode::Id("ptr".to_string())),
        })
    );
}

#[test]
fn test_parse_unary_address_of() {
    let tokens = vec![
        Token::And,
        Token::Id("var".to_string()),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_unary();
    assert_eq!(
        ast,
        Some(ASTNode::UnaryOp {
            op: Token::And,
            expr: Box::new(ASTNode::Id("var".to_string())),
        })
    );
}

#[test]
fn test_parse_unary_logical_not() {
    let tokens = vec![
        Token::Not,
        Token::Id("flag".to_string()),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_unary();
    assert_eq!(
        ast,
        Some(ASTNode::UnaryOp {
            op: Token::Not,
            expr: Box::new(ASTNode::Id("flag".to_string())),
        })
    );
}

#[test]
fn test_parse_prefix_increment() {
    let tokens = vec![
        Token::Inc,
        Token::Id("x".to_string()),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_unary();
    assert_eq!(
        ast,
        Some(ASTNode::UnaryOp {
            op: Token::Inc,
            expr: Box::new(ASTNode::Id("x".to_string())),
        })
    );
}

#[test]
fn test_parse_prefix_decrement() {
    let tokens = vec![
        Token::Dec,
        Token::Id("y".to_string()),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_unary();
    assert_eq!(
        ast,
        Some(ASTNode::UnaryOp {
            op: Token::Dec,
            expr: Box::new(ASTNode::Id("y".to_string())),
        })
    );
}

#[test]
fn test_simple_addition() {
    let tokens = vec![
        Token::Num(5),
        Token::Add,
        Token::Num(3),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_binary(0);
    assert_eq!(
        ast,
        Some(ASTNode::BinaryOp {
            op: Token::Add,
            left: Box::new(ASTNode::Num(5)),
            right: Box::new(ASTNode::Num(3)),
        })
    );
}

#[test]
fn test_precedence_mul_before_add() {
    let tokens = vec![
        Token::Num(5),
        Token::Add,
        Token::Num(3),
        Token::Mul,
        Token::Num(2),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_binary(0);
    assert_eq!(
        ast,
        Some(ASTNode::BinaryOp {
            op: Token::Add,
            left: Box::new(ASTNode::Num(5)),
            right: Box::new(ASTNode::BinaryOp {
                op: Token::Mul,
                left: Box::new(ASTNode::Num(3)),
                right: Box::new(ASTNode::Num(2)),
            })
        })
    );
}

#[test]
fn test_parse_binary_precedence() {
    let tokens = vec![
        Token::Num(10),
        Token::Mul,
        Token::Num(2),
        Token::Sub,
        Token::Num(5),
        Token::Div,
        Token::Num(10),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_binary(0);
    assert_eq!(
        ast,
        Some(ASTNode::BinaryOp {
            op: Token::Sub,
            left: Box::new(ASTNode::BinaryOp{
                op: Token::Mul,
                left: Box::new(ASTNode::Num(10)),
                right: Box::new(ASTNode::Num(2)),
            }),
            right: Box::new(ASTNode::BinaryOp{
                op: Token::Div,
                left: Box::new(ASTNode::Num(5)),
                right: Box::new(ASTNode::Num(10)),
            }),
        })
    )
}

#[test]
fn test_multiple_nested_parentheses() {
    let tokens = vec![
        Token::LParen,             // (
            Token::LParen,          // (
                Token::Num(1),      // 1
                Token::Add,         // +
                Token::Num(2),      // 2
            Token::RParen,          // )
            Token::Mul,             // *
            Token::LParen,          // (
                Token::Num(3),      // 3
                Token::Sub,         // -
                Token::Num(4),      // 4
            Token::RParen,          // )
        Token::RParen,              // )
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();
    assert_eq!(
        ast,
        Some(ASTNode::BinaryOp {
            op: Token::Mul,
            left: Box::new(ASTNode::BinaryOp {
                op: Token::Add,
                left: Box::new(ASTNode::Num(1)),
                right: Box::new(ASTNode::Num(2)),
            }),
            right: Box::new(ASTNode::BinaryOp {
                op: Token::Sub,
                left: Box::new(ASTNode::Num(3)),
                right: Box::new(ASTNode::Num(4)),
            }),
        })
    );
}

#[test]
fn test_simple_assignment() {
    let tokens = vec![
        Token::Id("x".to_string()),
        Token::Assign,
        Token::Num(42),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();
    assert_eq!(
        ast,
        Some(ASTNode::Assign {
            name: "x".to_string(),
            value: Box::new(ASTNode::Num(42)),
        })
    );
}

#[test]
fn test_assignment_with_expression() {
    let tokens = vec![
        Token::Id("y".to_string()),
        Token::Assign,
        Token::Num(1),
        Token::Add,
        Token::Num(2),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();
    assert_eq!(
        ast,
        Some(ASTNode::Assign {
            name: "y".to_string(),
            value: Box::new(ASTNode::BinaryOp {
                op: Token::Add,
                left: Box::new(ASTNode::Num(1)),
                right: Box::new(ASTNode::Num(2)),
            })
        })
    );
}

#[test]
fn test_assignment_with_string_literal() {
    let tokens = vec![
        Token::Id("x".to_string()),
        Token::Assign,
        Token::Str("hello world".to_string()),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();
    assert_eq!(
        ast,
        Some(ASTNode::Assign {
            name: "x".to_string(),
            value: Box::new(ASTNode::Str("hello world".to_string())),
        })
    );
}

#[test]
fn test_simple_conditional_expression() {
    let tokens = vec![
        Token::Id("a".to_string()),
        Token::Cond,  // ?
        Token::Num(1),
        Token::Colon, // :
        Token::Num(0),
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();
    assert_eq!(
        ast,
        Some(ASTNode::Cond {
            cond: Box::new(ASTNode::Id("a".to_string())),
            then_branch: Box::new(ASTNode::Num(1)),
            else_branch: Box::new(ASTNode::Num(0)),
        })
    );
}
