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

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
	Num(i64),                //Number
	Id(String),              //Identifier
	Str(String),
	Return(Box<ASTNode>),
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
	Sizeof {
		expr: Box<ASTNode>,
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
}

pub struct Parser {
	tokens: Vec<Token>,
	pos: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, pos: 0 }
	}

	fn current(&self) -> Option<&Token> {
		self.tokens.get(self.pos)
	}

	fn advance(&mut self) {
		if self.pos < self.tokens.len() {
			self.pos += 1;
		}
	}

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

	pub fn parse_stmt(&mut self) -> Option<ASTNode> {
		if self.current() == Some(&Token::LBrace) {
			self.advance(); // consume '{'
			let mut body = Vec::new();
	
			while self.current() != Some(&Token::RBrace) && self.current() != Some(&Token::EOF) {
				if self.current() == Some(&Token::Semicolon) {
					self.advance();
					continue;
				}
	
				if let Some(stmt) = self.parse_if()
					.or_else(|| self.parse_decl())
					.or_else(|| self.parse_expr())
				{
					body.push(stmt);
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
				args: body, // crude way to group — you can define a Block(Vec<ASTNode>) if preferred
			})
		} else if self.current() == Some(&Token::Return) {
			self.advance(); // consume 'return'
			let expr = self.parse_expr()?; // Parse the return expression
			if self.current() == Some(&Token::Semicolon) {
				self.advance(); // Consume ';'
			}
			Some(ASTNode::Return(Box::new(expr))) // Correct ASTNode
		} else {
			let stmt = self.parse_func_def()
				.or_else(|| self.parse_if())
				.or_else(|| self.parse_decl())
				.or_else(|| self.parse_expr())?;
	
			if self.current() == Some(&Token::Semicolon) {
				match &stmt {
					ASTNode::If { .. } => {
						// do NOT consume ; here — let parent handle it in context
					}
					_ => {
						self.advance();
					}
				}
			}
			Some(stmt)
		}
	}
	
	

	// pub fn parse_if(&mut self) -> Option<ASTNode> {
	// 	if self.current() != Some(&Token::If) {
	// 		return None;
	// 	}
	// 	self.advance(); // consume 'if'
	
	// 	if self.current() != Some(&Token::LParen) {
	// 		return None;
	// 	}
	// 	self.advance(); // consume '('
	
	// 	let cond = self.parse_expr()?; // parse condition
	
	// 	if self.current() != Some(&Token::RParen) {
	// 		return None;
	// 	}
	// 	self.advance(); // consume ')'
	
	// 	let then_branch = self.parse_stmt()?; // parse then statement
	
	// 	let else_branch = if self.current() == Some(&Token::Else) {
	// 		self.advance(); // consume 'else'
	// 		Some(Box::new(self.parse_stmt()?))
	// 	} else {
	// 		None
	// 	};
	
	// 	Some(ASTNode::If {
	// 		cond: Box::new(cond),
	// 		then_branch: Box::new(then_branch),
	// 		else_branch,
	// 	})
	// }
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
	
		let then_branch = self.parse_stmt()?;
	
		let else_branch = if self.current() == Some(&Token::Else) {
			self.advance(); // consume 'else'
			Some(Box::new(self.parse_stmt()?))
		} else {
			None
		};
	
		let node = ASTNode::If {
			cond: Box::new(cond),
			then_branch: Box::new(then_branch),
			else_branch,
		};

		Some(node)
	}
	
	

	pub fn parse_decl(&mut self) -> Option<ASTNode> {
		let typename = match self.current()? {
			Token::Int | Token::Char | Token::Float | Token::Double |
			Token::Void | Token::Short | Token::Long => self.current()?.clone(),
			_ => return None,
		};

		if self.pos == 0 {
			if let Some(Token::Id(_)) = self.tokens.get(self.pos + 1) {
				if self.tokens.get(self.pos + 2) == Some(&Token::LParen) {
					return None;
				}
			}
		}
	
		self.advance(); // move past typename
		let name = match self.current()? {
			Token::Id(n) => n.clone(),
			_ => return None,
		};
		self.advance();
	
		if self.current() == Some(&Token::Assign) {
			self.advance();
			let value = self.parse_expr()?;
			return Some(ASTNode::DeclAssign {
				typename,
				name,
				value: Box::new(value),
			});
		}
	
		Some(ASTNode::Decl { typename, name })
	}
	
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
	
			let param_type = match self.current()? {
				Token::Int | Token::Char | Token::Float | Token::Double |
				Token::Void | Token::Short | Token::Long => self.current()?.clone(),
				_ => {
					return None;
				}
			};
			self.advance();
	
			let param_name = match self.current()? {
				Token::Id(name) => name.clone(),
				_ => {
					return None;
				}
			};
			self.advance();
	
			params.push((param_type, param_name));
	
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
			if let Some(stmt) = self.parse_if()
			.or_else(|| self.parse_decl())
			.or_else(|| self.parse_expr())
			.or_else(|| self.parse_stmt())
			{		
				body.push(stmt);
				if self.current() == Some(&Token::Semicolon) {
					self.advance();
				}
			} else {
				self.pos = snapshot;
				break;
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

	// pub fn parse_program(&mut self) -> Vec<ASTNode> {
	// 	let mut nodes = Vec::new();
	
	// 	while self.current() != Some(&Token::EOF) {
	
	// 		if self.current() == Some(&Token::Semicolon) {
	// 			self.advance();
	// 			continue;
	// 		}
	
	// 		let snapshot = self.pos;
	
	// 		if let Some(stmt) = self.parse_func_def() {
	// 			nodes.push(stmt);
	// 			continue;
	// 		}
	
	// 		self.pos = snapshot;
	// 		if let Some(stmt) = self.parse_if() {
	// 			nodes.push(stmt);
	// 			if self.current() == Some(&Token::Semicolon) {
	// 				self.advance();
	// 			}
	// 			continue;
	// 		}
	
	// 		self.pos = snapshot;
	// 		if let Some(stmt) = self.parse_decl() {
	// 			nodes.push(stmt);
	// 			if self.current() == Some(&Token::Semicolon) {
	// 				self.advance();
	// 			}
	// 			continue;
	// 		}
	
	// 		self.pos = snapshot;
	// 		if let Some(stmt) = self.parse_expr() {
	// 			nodes.push(stmt);
	// 			if self.current() == Some(&Token::Semicolon) {
	// 				self.advance();
	// 			}
	// 			continue;
	// 		}
	// 		break;
	// 	}
	// 	nodes
	// }
	pub fn parse_program(&mut self) -> Vec<ASTNode> {
		let mut nodes = Vec::new();
	
		while self.current() != Some(&Token::EOF) {
			if self.current() == Some(&Token::Semicolon) {
				self.advance();
				continue;
			}
	
			let snapshot = self.pos;
	
			// Try function definition first
			if let Some(func_def) = self.parse_func_def() {
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
	
	
}
