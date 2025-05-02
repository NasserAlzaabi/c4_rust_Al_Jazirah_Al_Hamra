use c4::parser::*;
use c4::lexer::Token;


#[test]
fn test_parse_empty_function() {
    let tokens = vec![
        Token::Int,
        Token::Id("main".to_string()),
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = vec![
        ASTNode::FuncDef {
            return_type: Token::Int,
            name: "main".to_string(),
            params: vec![],
            body: vec![],
        }
    ];

    assert_eq!(ast, expected);
}

#[test]
fn test_parse_function_with_declaration() {
    let tokens = vec![
        Token::Int,
        Token::Id("main".to_string()),
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::Int,
        Token::Id("x".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = vec![
        ASTNode::FuncDef {
            return_type: Token::Int,
            name: "main".to_string(),
            params: vec![],
            body: vec![
                ASTNode::Decl {
                    typename: Token::Int,
                    name: "x".to_string(),
                }
            ],
        }
    ];

    assert_eq!(ast, expected);
}

#[test]
fn test_parse_function_with_two_params_and_body() {
    let tokens = vec![
        Token::Int,
        Token::Id("add".into()),
        Token::LParen,
        Token::Int,
        Token::Id("a".into()),
        Token::Comma,
        Token::Int,
        Token::Id("b".into()),
        Token::RParen,
        Token::LBrace,
        Token::Int,
        Token::Id("sum".into()),
        Token::Assign,
        Token::Id("a".into()),
        Token::Add,
        Token::Id("b".into()),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_func_def();

    let expected_ast = Some(ASTNode::FuncDef {
        return_type: Token::Int,
        name: "add".to_string(),
        params: vec![
            (Token::Int, "a".to_string()),
            (Token::Int, "b".to_string()),
        ],
        body: vec![
            ASTNode::DeclAssign {
                typename: Token::Int,
                name: "sum".to_string(),
                value: Box::new(ASTNode::BinaryOp {
                    op: Token::Add,
                    left: Box::new(ASTNode::Id("a".to_string())),
                    right: Box::new(ASTNode::Id("b".to_string())),
                }),
            },
        ],
    });

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_parse_function_with_no_params_and_return() {
    let tokens = vec![
        Token::Int,
        Token::Id("getValue".into()),
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::Int,
        Token::Id("x".into()),
        Token::Assign,
        Token::Num(100),
        Token::Semicolon,
        Token::Return,
        Token::Id("x".into()),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_func_def();

    let expected_ast = Some(ASTNode::FuncDef {
        return_type: Token::Int,
        name: "getValue".to_string(),
        params: vec![],
        body: vec![
            ASTNode::DeclAssign {
                typename: Token::Int,
                name: "x".to_string(),
                value: Box::new(ASTNode::Num(100)),
            },
            ASTNode::FuncCall {
                name: "return".into(),
                args: vec![ASTNode::Id("x".into())],
            },
        ],
    });

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_parse_simple_if() {
    let tokens = vec![
        Token::If,
        Token::LParen,
        Token::Num(1),
        Token::RParen,
        Token::Id("x".to_string()),
        Token::Assign,
        Token::Num(5),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = vec![
        ASTNode::If {
            cond: Box::new(ASTNode::Num(1)),
            then_branch: Box::new(ASTNode::Assign {
                name: "x".to_string(),
                value: Box::new(ASTNode::Num(5)),
            }),
            else_branch: None,
        }
    ];

    assert_eq!(ast, expected);
}

#[test]
fn test_parse_if_else() {
    let tokens = vec![
        Token::If,
        Token::LParen,
        Token::Id("x".to_string()),
        Token::Eq,
        Token::Num(0),
        Token::RParen,
        Token::Id("y".to_string()),
        Token::Assign,
        Token::Num(1),
        Token::Semicolon,
        Token::Else,
        Token::Id("y".to_string()),
        Token::Assign,
        Token::Num(2),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = vec![
        ASTNode::If {
            cond: Box::new(ASTNode::BinaryOp {
                op: Token::Eq,
                left: Box::new(ASTNode::Id("x".to_string())),
                right: Box::new(ASTNode::Num(0)),
            }),
            then_branch: Box::new(ASTNode::Assign {
                name: "y".to_string(),
                value: Box::new(ASTNode::Num(1)),
            }),
            else_branch: Some(Box::new(ASTNode::Assign {
                name: "y".to_string(),
                value: Box::new(ASTNode::Num(2)),
            })),
        }
    ];

    assert_eq!(ast, expected);
}

#[test]
fn test_if_with_block_then() {
	let tokens = vec![
		Token::If,
		Token::LParen,
		Token::Num(1),
		Token::RParen,
		Token::LBrace,
		Token::Id("a".to_string()),
		Token::Assign,
		Token::Num(1),
		Token::Semicolon,
		Token::Id("b".to_string()),
		Token::Assign,
		Token::Num(2),
		Token::Semicolon,
		Token::RBrace,
		Token::EOF,
	];

	let mut parser = Parser::new(tokens);
	let ast = parser.parse_program();

	let expected = vec![
		ASTNode::If {
			cond: Box::new(ASTNode::Num(1)),
			then_branch: Box::new(ASTNode::FuncCall {
				name: "__block".to_string(),
				args: vec![
					ASTNode::Assign {
						name: "a".to_string(),
						value: Box::new(ASTNode::Num(1)),
					},
					ASTNode::Assign {
						name: "b".to_string(),
						value: Box::new(ASTNode::Num(2)),
					},
				],
			}),
			else_branch: None,
		},
	];

	assert_eq!(ast, expected);
}

#[test]
fn test_nested_if_else() {
	let tokens = vec![
		Token::If,
		Token::LParen,
		Token::Id("x".to_string()),
		Token::Gt,
		Token::Num(0),
		Token::RParen,
		Token::If,
		Token::LParen,
		Token::Id("y".to_string()),
		Token::Lt,
		Token::Num(5),
		Token::RParen,
		Token::Id("z".to_string()),
		Token::Assign,
		Token::Num(1),
		Token::Semicolon,
		Token::Else,
		Token::Id("z".to_string()),
		Token::Assign,
		Token::Num(2),
		Token::Semicolon,
		Token::EOF,
	];

	let mut parser = Parser::new(tokens);
	let ast = parser.parse_program();

	let expected = vec![
		ASTNode::If {
			cond: Box::new(ASTNode::BinaryOp {
				op: Token::Gt,
				left: Box::new(ASTNode::Id("x".to_string())),
				right: Box::new(ASTNode::Num(0)),
			}),
			then_branch: Box::new(ASTNode::If {
				cond: Box::new(ASTNode::BinaryOp {
					op: Token::Lt,
					left: Box::new(ASTNode::Id("y".to_string())),
					right: Box::new(ASTNode::Num(5)),
				}),
				then_branch: Box::new(ASTNode::Assign {
					name: "z".to_string(),
					value: Box::new(ASTNode::Num(1)),
				}),
				else_branch: Some(Box::new(ASTNode::Assign {
					name: "z".to_string(),
					value: Box::new(ASTNode::Num(2)),
				})),
			}),
			else_branch: None,
		}
	];

	assert_eq!(ast, expected);
}

#[test]
fn test_else_if_chain_complete() {
	let tokens = vec![
		Token::If,
		Token::LParen,
		Token::Id("x".to_string()),
		Token::Lt,
		Token::Num(0),
		Token::RParen,
		Token::Id("y".to_string()),
		Token::Assign,
		Token::Num(10),
		Token::Semicolon,
		Token::Else,
		Token::If,
		Token::LParen,
		Token::Id("x".to_string()),
		Token::Gt,
		Token::Num(0),
		Token::RParen,
		Token::Id("y".to_string()),
		Token::Assign,
		Token::Num(20),
		Token::Semicolon,
		Token::Else,
		Token::Id("y".to_string()),
		Token::Assign,
		Token::Num(30),
		Token::Semicolon,
		Token::EOF,
	];

	let mut parser = Parser::new(tokens);
	let ast = parser.parse_program();

	let expected = vec![
		ASTNode::If {
			cond: Box::new(ASTNode::BinaryOp {
				op: Token::Lt,
				left: Box::new(ASTNode::Id("x".to_string())),
				right: Box::new(ASTNode::Num(0)),
			}),
			then_branch: Box::new(ASTNode::Assign {
				name: "y".to_string(),
				value: Box::new(ASTNode::Num(10)),
			}),
			else_branch: Some(Box::new(ASTNode::If {
				cond: Box::new(ASTNode::BinaryOp {
					op: Token::Gt,
					left: Box::new(ASTNode::Id("x".to_string())),
					right: Box::new(ASTNode::Num(0)),
				}),
				then_branch: Box::new(ASTNode::Assign {
					name: "y".to_string(),
					value: Box::new(ASTNode::Num(20)),
				}),
				else_branch: Some(Box::new(ASTNode::Assign {
					name: "y".to_string(),
					value: Box::new(ASTNode::Num(30)),
				})),
			})),
		}
	];

	assert_eq!(ast, expected);
}



#[test]
fn test_parse_simple_declaration_assignment() {
    let tokens = vec![
        Token::Int,
        Token::Id("x".to_string()),
        Token::Assign,
        Token::Num(10),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let expected = vec![
        ASTNode::DeclAssign {
            typename: Token::Int,
            name: "x".to_string(),
            value: Box::new(ASTNode::Num(10)),
        }
    ];

    assert_eq!(ast, expected);
}


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
