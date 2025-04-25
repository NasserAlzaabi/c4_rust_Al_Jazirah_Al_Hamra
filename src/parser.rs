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
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Num(i64),
	Id(String),
	Str(String),

	Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne,
	Lt, Gt, Le, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod,
	Inc, Dec, Brak, Not,

	Semicolon, Colon, Comma,
	LParen, RParen, LBrace, RBrace,
	LBracket, RBracket,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
	Num(i64),                //Number
	Id(String),              //Identifier
	Str(String),
	UnaryOp {                //Unary Operator like '-', '*', '&', '!'
		op: Token,
		expr: Box<ASTNode>,
	},
	BinaryOp {               //Binary Operator like '+', '==', '*'
		op: Token,
		left: Box<ASTNode>,
		right: Box<ASTNode>,
	},
	FuncCall {               //Function call like f(x, y)
		name: String,
		args: Vec<ASTNode>,
	},
	Sizeof {                 //sizeof(int) or sizeof(x)
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
}

pub struct Parser {
	tokens: Vec<Token>,     //list of tokens (Temp before git merge with lexer) 
	pos: usize,             //Current token position
}

impl<'a> Parser<> {
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
					self.advance(); //skip '('
					let mut args = Vec::new();
					while self.current() != Some(&Token::RParen) {
						if let Some(arg) = self.parse_primary() {
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
				self.advance(); //skip '('
				let expr = self.parse_expr(); //Full expression inside ()
				if self.current() == Some(&Token::RParen) {
					self.advance(); //skip ')'
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
}
