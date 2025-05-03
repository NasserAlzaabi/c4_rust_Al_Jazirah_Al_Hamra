use c4::lexer::*;
use c4::parser::*;
use c4::vm::*;

#[test]
fn test_parse_simple_decl() {
    let tokens = vec![
        Token::Int,
        Token::Id("x".into()),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::Block(vec![
        ASTNode::Decl {
            typename: Token::Int,
            name: "x".into(),
        },
    ]);

    assert_eq!(ast, vec![expected]);
}

#[test]
fn test_parse_decl_assign() {
    let tokens = vec![
        Token::Int,
        Token::Id("x".into()),
        Token::Assign,
        Token::Num(10),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::Block(vec![
        ASTNode::DeclAssign {
            typename: Token::Int,
            name: "x".into(),
            value: Box::new(ASTNode::Num(10)),
        },
    ]);

    assert_eq!(ast, vec![expected]);
}

#[test]
fn test_parse_multiple_decls() {
    let tokens = vec![
        Token::Int,
        Token::Id("x".into()),
        Token::Assign,
        Token::Num(1),
        Token::Comma,
        Token::Id("y".into()),
        Token::Comma,
        Token::Id("z".into()),
        Token::Assign,
        Token::Num(3),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::Block(vec![
        ASTNode::DeclAssign {
            typename: Token::Int,
            name: "x".into(),
            value: Box::new(ASTNode::Num(1)),
        },
        ASTNode::Decl {
            typename: Token::Int,
            name: "y".into(),
        },
        ASTNode::DeclAssign {
            typename: Token::Int,
            name: "z".into(),
            value: Box::new(ASTNode::Num(3)),
        },
    ]);

    assert_eq!(ast, vec![expected]);
}

#[test]
fn test_parse_if_else() {
    let tokens = vec![
        Token::If,
        Token::LParen,
        Token::Id("x".into()),
        Token::Eq,
        Token::Num(0),
        Token::RParen,
        Token::LBrace,
        Token::Id("y".into()),
        Token::Assign,
        Token::Num(1),
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Id("y".into()),
        Token::Assign,
        Token::Num(2),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::If {
        cond: Box::new(ASTNode::BinaryOp {
            op: Token::Eq,
            left: Box::new(ASTNode::Id("x".into())),
            right: Box::new(ASTNode::Num(0)),
        }),
        then_branch: Box::new(ASTNode::Block(vec![ASTNode::Assign {
            name: "y".into(),
            value: Box::new(ASTNode::Num(1)),
        }])),
        else_branch: Some(Box::new(ASTNode::Block(vec![ASTNode::Assign {
            name: "y".into(),
            value: Box::new(ASTNode::Num(2)),
        }]))),
    };

    assert_eq!(ast, vec![expected]);
}


#[test]
fn test_parse_while_loop() {
    let tokens = vec![
        Token::While,
        Token::LParen,
        Token::Id("x".into()),
        Token::Gt,
        Token::Num(0),
        Token::RParen,
        Token::LBrace,
        Token::Id("x".into()),
        Token::Assign,
        Token::Id("x".into()),
        Token::Sub,
        Token::Num(1),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::WhileLoop {
            condition: Box::new(ASTNode::BinaryOp {
                op: Token::Gt,
                left: Box::new(ASTNode::Id("x".into())),
                right: Box::new(ASTNode::Num(0)),
            }),
            body: vec![
                ASTNode::Assign {
                    name: "x".into(),
                    value: Box::new(ASTNode::BinaryOp {
                        op: Token::Sub,
                        left: Box::new(ASTNode::Id("x".into())),
                        right: Box::new(ASTNode::Num(1)),
                    }),
                },
            ],
        };

    assert_eq!(ast, vec![expected]);
}

#[test]
fn test_parse_function_def() {
    let tokens = vec![
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

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    assert!(matches!(ast[0], ASTNode::FuncDef { .. }));
}

#[test]
fn test_parse_function_decl() {
    let tokens = vec![
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

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::FuncDef {
        return_type: Token::Int,
        name: "main".into(),
        params: vec![],
        body: vec![
            ASTNode::Return(Box::new(ASTNode::Num(0))),
        ],
    };

    assert_eq!(ast, vec![expected]);
}

#[test]
fn test_nested_if_else() {
    let tokens = vec![
        Token::If,
        Token::LParen,
        Token::Id("a".into()),
        Token::Eq,
        Token::Num(1),
        Token::RParen,
        Token::LBrace,
        Token::If,
        Token::LParen,
        Token::Id("b".into()),
        Token::Eq,
        Token::Num(2),
        Token::RParen,
        Token::LBrace,
        Token::Id("c".into()),
        Token::Assign,
        Token::Num(3),
        Token::Semicolon,
        Token::RBrace,
        Token::RBrace,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::If {
        cond: Box::new(ASTNode::BinaryOp {
            op: Token::Eq,
            left: Box::new(ASTNode::Id("a".into())),
            right: Box::new(ASTNode::Num(1)),
        }),
        then_branch: Box::new(ASTNode::Block(vec![
            ASTNode::If {
                cond: Box::new(ASTNode::BinaryOp {
                    op: Token::Eq,
                    left: Box::new(ASTNode::Id("b".into())),
                    right: Box::new(ASTNode::Num(2)),
                }),
                then_branch: Box::new(ASTNode::Block(vec![
                    ASTNode::Assign {
                        name: "c".into(),
                        value: Box::new(ASTNode::Num(3)),
                    }
                ])),
                else_branch: None,
            }
        ])),
        else_branch: None,
    };

    assert_eq!(ast, vec![expected]);
}

#[test]
fn test_mixed_multi_decls() {
    let tokens = vec![
        Token::Int,
        Token::Id("a".into()),
        Token::Assign,
        Token::Num(1),
        Token::Comma,
        Token::Id("b".into()),
        Token::Comma,
        Token::Id("c".into()),
        Token::Assign,
        Token::Num(3),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = ASTNode::Block(vec![
        ASTNode::DeclAssign {
            typename: Token::Int,
            name: "a".into(),
            value: Box::new(ASTNode::Num(1)),
        },
        ASTNode::Decl {
            typename: Token::Int,
            name: "b".into(),
        },
        ASTNode::DeclAssign {
            typename: Token::Int,
            name: "c".into(),
            value: Box::new(ASTNode::Num(3)),
        },
    ]);

    assert_eq!(ast, vec![expected]);
}

#[test]
fn test_while_with_block() {
    let tokens = vec![
        Token::While,
        Token::LParen,
        Token::Id("x".into()),
        Token::Lt,
        Token::Num(10),
        Token::RParen,
        Token::LBrace,
        Token::Id("x".into()),
        Token::Assign,
        Token::Id("x".into()),
        Token::Add,
        Token::Num(1),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected =
        ASTNode::WhileLoop {
            condition: Box::new(ASTNode::BinaryOp {
                op: Token::Lt,
                left: Box::new(ASTNode::Id("x".into())),
                right: Box::new(ASTNode::Num(10)),
            }),
            body: vec![
                ASTNode::Assign {
                    name: "x".into(),
                    value: Box::new(ASTNode::BinaryOp {
                        op: Token::Add,
                        left: Box::new(ASTNode::Id("x".into())),
                        right: Box::new(ASTNode::Num(1)),
                    }),
                }
            ],
        };

    assert_eq!(ast, vec![expected]);
}
