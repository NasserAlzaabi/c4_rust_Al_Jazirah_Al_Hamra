// Parser file

/*
Parses the tokens into proper syntax, checks if the line is written grammatically
----TODO----
Parser struct to read tokens
AST to represent:
Numbers,
Variables,
Operations,
Function calls,
Expressions,
Assignments,
Conditions
------------
Parsing types to complete:
Primary parsing - Nums, Id, Parenthases, Funcall
Unary parsing - Op with one variable
Binary parsing - Op with two variables
Assignment parsing - variable assignment "="
Conditional parsing - Ternary conditional "? : "
*/

/* TEMPORARY LEXER OUTPUT*/
use crate::lexer::Token;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token {
// 	Num(i64),
// 	Id(String),
// 	Str(String),

// 	Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne,
// 	Lt, Gt, Le, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod,
// 	Inc, Dec, Brak, Not,

// 	Semicolon, Colon, Comma,
// 	LParen, RParen, LBrace, RBrace,
// 	LBracket, RBracket,
// }

/// Represents a node in the Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
	Num(i64),                //Number
	Id(String),              //Identifier
	Str(String),
	Return(Box<ASTNode>),
	Block(Vec<ASTNode>),
	UnaryOp {
		op: Token,
		expr: Box<ASTNode>,
	},
	BinaryOp {
		op: Token,
		left: Box<ASTNode>,
		right: Box<ASTNode>,
	},
	FuncCall {
		name: String,
		args: Vec<ASTNode>,
	},
	FuncDef {
		return_type: Token,
		name: String,
		params: Vec<(Token, String)>, // e.g., int x, float y
		body: Vec<ASTNode>,
	},
	Assign {
		name: String,
		value: Box<ASTNode>,
	},
	Cond {
		cond: Box<ASTNode>,
		then_branch: Box<ASTNode>,
		else_branch: Box<ASTNode>,
	},
	If {
		cond: Box<ASTNode>,
		then_branch: Box<ASTNode>,
		else_branch: Option<Box<ASTNode>>,
	},
	Decl {
		typename: Token,
		name: String,
	},
	DeclAssign {
		typename: Token,
		name: String,
		value: Box<ASTNode>,
	},
	WhileLoop {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
}

/// Parser for converting tokens into an Abstract Syntax Tree
pub struct Parser {
	tokens: Vec<Token>,
	pos: usize,
	pending_decls: Vec<ASTNode>,
}

impl Parser {
	/// Creates a new Parser with the given tokens
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser {
			tokens,
			pos: 0,
			pending_decls: Vec::new(),
		}
	}

	/// Returns the current token without advancing
	fn current(&self) -> Option<&Token> {
		self.tokens.get(self.pos)
	}

	/// Moves to the next token
	fn advance(&mut self) {
		if self.pos < self.tokens.len() {
			self.pos += 1;
		}
	}

	/// Expects a specific token and advances if found, panics otherwise
	fn expect(&mut self, expected: Token) {
        if self.current() == Some(&expected) {
            self.advance(); // Move to the next token
        } else {
            panic!(
                "Expected token {:?}, but found {:?}",
                expected, self.current()
            );
        }
    }

	/// Parses primary expressions: numbers, identifiers, function calls, strings, and parenthesized expressions
	pub fn parse_primary(&mut self) -> Option<ASTNode> {
		match self.current() {
			Some(Token::Num(value)) => {
				let node = ASTNode::Num(*value);
				self.advance();
				Some(node)
			}
			Some(Token::Id(name)) => {
				let name = name.clone();
				self.advance();
				if self.current() == Some(&Token::LParen) {
					self.advance();
					let mut args = Vec::new();
					while self.current() != Some(&Token::RParen) {
						if let Some(arg) = self.parse_expr() {
							args.push(arg);
						} else {
							return None;
						}
						if self.current() == Some(&Token::Comma) {
							self.advance();
						} else {
							break;
						}
					}
					if self.current() == Some(&Token::RParen) {
						self.advance();
						Some(ASTNode::FuncCall { name, args })
					} else {
						None
					}
				} else {
					Some(ASTNode::Id(name))
				}
			}
			Some(Token::Str(s)) => {
				let s = s.clone();
				self.advance();
				Some(ASTNode::Str(s))
			}
			Some(Token::LParen) => {
				self.advance();
				let expr = self.parse_expr();
				if self.current() == Some(&Token::RParen) {
					self.advance();
					expr
				} else {
					None
				}
			}
			_ => None,
		}
	}

	/// Parses unary operations like -x, *x, &x, !x, ++x, --x
	pub fn parse_unary(&mut self) -> Option<ASTNode> {
		match self.current() {
			Some(Token::Sub) | Some(Token::Mul) | Some(Token::And) |
			Some(Token::Not) | Some(Token::Inc) | Some(Token::Dec) => {
				let op = self.current().cloned().unwrap();
				self.advance();
				if let Some(expr) = self.parse_unary() {
					Some(ASTNode::UnaryOp {
						op,
						expr: Box::new(expr),
					})
				} else {
					None
				}
			}
			_ => self.parse_primary(),
		}
	}

	/// Returns the precedence level of an operator
	fn precedence(op: &Token) -> u8 {
		match op {
			Token::Lor => 1,   // ||
			Token::Lan => 2,   // &&
			Token::Eq | Token::Ne => 3,   // ==, !=
			Token::Lt | Token::Gt | Token::Le | Token::Ge => 4,   // <, >, <=, >=
			Token::Add | Token::Sub => 5,  // +, -
			Token::Mul | Token::Div | Token::Mod => 6, // *, /, %
			_ => 0,
		}
	}

	/// Parses binary operations with proper operator precedence
	pub fn parse_binary(&mut self, min_prec: u8) -> Option<ASTNode> {
		let mut left = self.parse_unary()?;
	
		while let Some(op) = self.current().cloned() {
			let prec = Self::precedence(&op);
			if prec < min_prec || prec == 0 {
				break;
			}
	
			self.advance();
			let mut right = self.parse_binary(prec + 1)?;
	
			while let Some(next_op) = self.current() {
				let next_prec = Self::precedence(next_op);
				if next_prec > prec {
					right = self.parse_binary(next_prec)?;
				} else {
					break;
				}
			}
	
			left = ASTNode::BinaryOp {
				op,
				left: Box::new(left),
				right: Box::new(right),
			};
		}
		Some(left)
	}

	/// Parses statements: blocks, return statements, and expressions
	pub fn parse_stmt(&mut self) -> Option<ASTNode> {
		println!("parse_stmt: token = {:?}", self.current());
		if let Some(decl) = self.pending_decls.pop() {
			return Some(decl);
		}
		if self.current() == Some(&Token::LBrace) {
			self.advance(); // consume '{'
			let mut body = Vec::new();
	
			while self.current() != Some(&Token::RBrace) && self.current() != Some(&Token::EOF) {
				if self.current() == Some(&Token::Semicolon) {
					self.advance();
					continue;
				}
	
				// if let Some(stmt) = self.parse_if()
				// 	.or_else(|| self.parse_while())
				// 	.or_else(|| self.parse_decl())
				// 	.or_else(|| self.parse_expr())
				// {
				// 	body.push(stmt);
				if let Some(stmt) = self.parse_if()
				.or_else(|| self.parse_while())
				.or_else(|| self.parse_decl())
				.or_else(|| self.parse_expr())
				{
					match stmt {
						ASTNode::Block(stmts) => {
							self.pending_decls.extend(stmts.into_iter().rev()); // store remaining
							if let Some(first) = self.pending_decls.pop() {
								body.push(first);
							}
						}
						_ => {
							body.push(stmt);
						}
					}
					if self.current() == Some(&Token::Semicolon) {
						self.advance();
					}
				} else {
					break;
				}
			}
	
			if self.current() == Some(&Token::RBrace) {
				self.advance(); // consume '}'
			}
	
			// Wrap body in a block node (optional)
			Some(ASTNode::FuncCall {
				name: "__block".into(),
				args: body,
			})
		} else if self.current() == Some(&Token::Return) {
			self.advance(); // consume 'return'
			
			// Handle return with no expression (void functions)
			if self.current() == Some(&Token::Semicolon) {
				self.advance(); // Consume ';'
				return Some(ASTNode::Return(Box::new(ASTNode::Num(0)))); // Return 0 as default
			}
			
			let expr = self.parse_expr()?; // Parse the return expression
			if self.current() == Some(&Token::Semicolon) {
				self.advance(); // Consume ';'
			}
			Some(ASTNode::Return(Box::new(expr)))
		} else {
			let stmt = self.parse_func_def()
				.or_else(|| self.parse_if())
				.or_else(|| self.parse_while()) // ADD THIS LINE
				.or_else(|| self.parse_decl())
				.or_else(|| self.parse_expr())?;
	
			if self.current() == Some(&Token::Semicolon) {
				match &stmt {
					ASTNode::If { .. } | ASTNode::WhileLoop { .. } => {
						// Do NOT consume ';' — block handles it
					}
					_ => {
						self.advance();
					}
				}
			}
			Some(stmt)
		}
	}	
	
	/// Parses if-else statements
	pub fn parse_if(&mut self) -> Option<ASTNode> {
		if self.current() != Some(&Token::If) {
			return None;
		}
		self.advance(); // consume 'if'
	
		if self.current() != Some(&Token::LParen) {
			return None;
		}
		self.advance(); // consume '('
	
		let cond = self.parse_expr()?;
	
		if self.current() != Some(&Token::RParen) {
			return None;
		}
		self.advance(); // consume ')'
	
		let then_branch = self.parse_block()?; // Parse the 'then' branch as a block
		let else_branch = if self.current() == Some(&Token::Else) {
			self.advance(); // consume 'else'
			Some(self.parse_block()?) // Parse the 'else' branch as a block
		} else {
			None
		};
	
		Some(ASTNode::If {
			cond: Box::new(cond),
			then_branch: Box::new(then_branch),
			else_branch: else_branch.map(Box::new), // Wrap else_branch in a Box
		})
	}
	
	/// Parses while loops
	pub fn parse_while(&mut self) -> Option<ASTNode> {
	    if self.current() == Some(&Token::While) {
	        self.advance(); // Consume 'while'
	        self.expect(Token::LParen); // Expect '('
	        let condition = self.parse_expr()?; // Parse condition
	        self.expect(Token::RParen); // Expect ')'

	        // Parse the loop body
	        let body = match self.parse_block()? {
	            ASTNode::Block(statements) => statements, // Extract the Vec<ASTNode> from the Block
	            _ => return None,
	        };

	        Some(ASTNode::WhileLoop {
	            condition: Box::new(condition),
	            body,
	        })
	    } else {
	        None
	    }
	}	

	/// Parses variable declarations
	pub fn parse_decl(&mut self) -> Option<ASTNode> {
	    let typename = match self.current()? {
	        Token::Int | Token::Char => self.current()?.clone(),
	        _ => return None,
	    };

	    self.advance(); // Move past the type (e.g., `char`)

	    // Check for pointer (`*`)
	    let is_pointer = if self.current() == Some(&Token::Mul) {
	        self.advance(); // Consume `*`
	        true
	    } else {
	        false
	    };

	    let mut decls = Vec::new();

	    loop {
	        let name = match self.current()? {
	            Token::Id(n) => n.clone(),
	            _ => return None,
	        };
	        self.advance(); // Consume the identifier

	        // Check for assignment
	        if self.current() == Some(&Token::Assign) {
	            self.advance(); // Consume `=`
	            let value = self.parse_expr()?;
	            decls.push(ASTNode::DeclAssign {
	                typename: if is_pointer {
	                    Token::CharPointer // Use a custom token for `char*`
	                } else {
	                    typename.clone()
	                },
	                name,
	                value: Box::new(value),
	            });
	        } else {
	            // Variable declaration without assignment
	            decls.push(ASTNode::Decl {
	                typename: if is_pointer {
	                    Token::CharPointer
	                } else {
	                    typename.clone()
	                },
	                name,
	            });
	        }

	        // Check for comma or semicolon
	        match self.current() {
	            Some(Token::Comma) => self.advance(), // Continue to the next declaration
	            Some(Token::Semicolon) => {
	                self.advance(); // End of declaration
	                break;
	            }
	            _ => return None,
	        }
	    }

	    Some(ASTNode::Block(decls))
	}
	
	/// Parses function definitions
	pub fn parse_func_def(&mut self) -> Option<ASTNode> {	
		let return_type = match self.current()? {
			Token::Int | Token::Char | Token::Float | Token::Double |
			Token::Void | Token::Short | Token::Long => self.current()?.clone(),
			_ => return None,
		};
		self.advance();
	
		let name = match self.current()? {
			Token::Id(name) => name.clone(),
			_ => return None,
		};
		self.advance();
	
		if self.current() != Some(&Token::LParen) {
			return None;
		}
		self.advance();
	
		let mut params = Vec::new();
	
		while self.current() != Some(&Token::RParen) {
			// Get parameter type
			let param_type = match self.current()? {
				Token::Int | Token::Char | Token::Float | Token::Double |
				Token::Void | Token::Short | Token::Long => self.current()?.clone(),
				_ => {
					return None;
				}
			};
			self.advance();
			
			// Check for pointer (*) after type
			let mut is_pointer = false;
			if self.current() == Some(&Token::Mul) {
				is_pointer = true;
				self.advance();
			}
	
			// Get parameter name
			let param_name = match self.current()? {
				Token::Id(name) => name.clone(),
				_ => {
					return None;
				}
			};
			self.advance();
			
			// Store the appropriate type (original or pointer version)
			if is_pointer {
				// Use CharPointer for char* parameters
				if param_type == Token::Char {
					params.push((Token::CharPointer, param_name));
				} else {
					// For other pointer types (not fully implemented yet)
					params.push((param_type, param_name));
				}
			} else {
				params.push((param_type, param_name));
			}
	
			if self.current() == Some(&Token::Comma) {
				self.advance();
			} else {
				break;
			}
		}
	
		if self.current() != Some(&Token::RParen) {
			return None;
		}
		self.advance();
	
		if self.current() != Some(&Token::LBrace) {
			return None;
		}
		self.advance();
	
		let mut body = Vec::new();
		while self.current() != Some(&Token::RBrace) && self.current() != Some(&Token::EOF) {
			if self.current() == Some(&Token::Semicolon) {
				self.advance();
				continue;
			}
	
			let snapshot = self.pos;
			
			// Try to parse a statement - wrap in a match to handle failures gracefully
			match self.parse_if()
				.or_else(|| self.parse_decl())
				.or_else(|| self.parse_expr())
				.or_else(|| self.parse_stmt()) {
				Some(stmt) => {
					body.push(stmt);
					if self.current() == Some(&Token::Semicolon) {
						self.advance();
					}
				},
				None => {
					// In case of failure, try to skip to the next statement
					println!("Warning: Failed to parse statement at position {}, skipping.", snapshot);
					self.pos = snapshot;
					
					// Skip until semicolon or right brace to recover
					while self.current() != Some(&Token::Semicolon) && 
						  self.current() != Some(&Token::RBrace) && 
						  self.current() != Some(&Token::EOF) {
						self.advance();
					}
					if self.current() == Some(&Token::Semicolon) {
						self.advance(); // Skip the semicolon
					}
				}
			}
		}
	
		if self.current() != Some(&Token::RBrace) {
			return None;
		}
		self.advance();
	
		Some(ASTNode::FuncDef {
			return_type,
			name,
			params,
			body,
		})
	}
	
	/// Parses expressions including assignments and ternary conditionals
	pub fn parse_expr(&mut self) -> Option<ASTNode> {
		let node = self.parse_binary(0)?;
	
		if self.current() == Some(&Token::Assign) {
			self.advance();
			if let ASTNode::Id(name) = node {
				let value = self.parse_expr()?;
				return Some(ASTNode::Assign {
					name,
					value: Box::new(value),
				});
			} else {
				//Assignment target must be an identifier
				return None;
			}
		}

		if self.current() == Some(&Token::Cond) { //Token::Cond is '?'
    	    self.advance();
    	    let then_branch = self.parse_expr()?;

    	    if self.current() != Some(&Token::Colon) { //':' is expected
    	        return None;
    	    }
    	    self.advance();
    	    let else_branch = self.parse_expr()?;

    	    return Some(ASTNode::Cond {
    	        cond: Box::new(node),
    	        then_branch: Box::new(then_branch),
    	        else_branch: Box::new(else_branch),
    	    });
    	}
		Some(node)
	}

	/// Parses a complete program
	pub fn parse_program(&mut self) -> Vec<ASTNode> {
		let mut nodes = Vec::new();
	
		while self.current() != Some(&Token::EOF) {
			if self.current() == Some(&Token::Semicolon) {
				self.advance();
				continue;
			}
	
			let snapshot = self.pos;
	
			// Try function definition first
			let result = self.parse_func_def();
			if let Some(func_def) = result {
				nodes.push(func_def);
				continue;
			}
	
			self.pos = snapshot;
	
			// Try top-level variable declaration (like: int x = 10;)
			if let Some(decl) = self.parse_decl() {
				nodes.push(decl);
				if self.current() == Some(&Token::Semicolon) {
					self.advance();
				}
				continue;
			}
	
			self.pos = snapshot;
	
			// Try statement (includes if, return, expr, block, etc.)
			if let Some(stmt) = self.parse_stmt() {
				nodes.push(stmt);
				if self.current() == Some(&Token::Semicolon) {
					self.advance();
				}
				continue;
			}
	
			// Could not parse anything, break
			self.pos = snapshot;
			break;
		}
	
		nodes
	}

    /// Parses a block of code enclosed in curly braces
    fn parse_block(&mut self) -> Option<ASTNode> {
        if self.current() != Some(&Token::LBrace) {
            return None; // Expect '{' to start a block
        }
        self.advance(); // Consume '{'

        let mut stmts = Vec::new();
        while self.current() != Some(&Token::RBrace) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                return None; // Invalid statement
            }
        }
        self.advance(); // Consume '}'

        Some(ASTNode::Block(stmts))
    }
}
