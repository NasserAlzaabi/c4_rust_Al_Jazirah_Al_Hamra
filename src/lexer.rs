//Tokinizer file

/*
----TODO----
Read characters from source and categorize them as tokens.
Handle:
Whitespace and comments
Identifiers and Keywords
Numeric literals like decimal, hex, octal
String and char literals
Operators like ==, <=, ||
*/

/// Represents the different types of tokens in the C language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	Num(i64),	//Numeric literal
	Id(String),	//Identifier
	Str(String), //String literals

	//Keywords
	Char, 
	CharPointer,
	Else, Enum, If, Int, Return, Sizeof, While,
	Void, Float, Double, Short, Long,

	//Operators
	Assign,  // =
	Cond,    // ?
	Lor,     // ||
	Lan,     // &&
	Or,      // |
	Xor,     // ^
	And,     // &
	Eq,      // ==
	Ne,      // !=
	Lt,      // <
	Gt,      // >
	Le,      // <=
	Ge,      // >=
	Shl,     // <<
	Shr,     // >>
	Add,     // +
	Sub,     // -
	Mul,     // *
	Div,     // /
	Mod,     // %
	Inc,     // ++
	Dec,     // --

	//Symbols and Punctuation
	Semicolon,  // ;
	Colon,      // :
	Comma,      // ,
	LParen,     // (
	RParen,     // )
	LBrace,     // {
	RBrace,     // }
	LBracket,   // [
	RBracket,   // ]
	Quote,      // '
	DQuote,     // "
	Hash,       // #
	Not,        // !
	Tilde,      // ~
	EOF,        // End of file

	Unknown(char), //unknown character
}

/// Lexical analyzer that converts source code into tokens
pub struct Lexer<'a> {
	pub chars: std::str::Chars<'a>, //iterator over the source
	pub current_char: Option<char>, //current character
	pub line: usize, //current line number
	pub peeked: Option<char>, //one-character lookahead
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given source code
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let current_char = chars.next();

        Lexer {
            chars,
            current_char,
            line: 1,
            peeked: None,
        }
    }

    /// Advance to the next character
    fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    /// Peek the next character without advancing
    fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.clone().next();
        }
        self.peeked
    }

    /// Consume the peeked character if any
    fn consume_peek(&mut self) {
        if let Some(c) = self.peeked {
            self.current_char = Some(c);
            self.chars.next();
            self.peeked = None;
        } else {
            self.advance();
        }
    }

    /// Skips whitespace characters and comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.current_char {
                Some(' ' | '\t' | '\r') => self.advance(),
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('#') => {
                    // Skip until end of line
                    while let Some(c) = self.current_char {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                Some('/') => {
                    if self.peek() == Some('/') {
                        self.consume_peek(); // consume the second slash
                        self.advance(); // move past the second slash
                        while let Some(c) = self.current_char {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    /// Determines if an identifier is a keyword or normal identifier
    fn keyword_or_id(ident: &str) -> Token {
        match ident {
            "char"    => Token::Char,
            "else"    => Token::Else,
            "enum"    => Token::Enum,
            "if"      => Token::If,
            "int"     => Token::Int,
            "return"  => Token::Return,
            "sizeof"  => Token::Sizeof,
            "while"   => Token::While,
            "void"    => Token::Void,
            "float"   => Token::Float,
            "double"  => Token::Double,
            "short"   => Token::Short,
            "long"    => Token::Long,
            _ => Token::Id(ident.to_string()),
        }
    }

    /// Returns the next token from the source code
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments(); //handle white space and comments

        match self.current_char {
            None => Token::EOF,

			Some(c) if c.is_ascii_alphabetic() || c == '_' => { //handle keyword and identifiers
				let mut ident = String::new();
	
				while let Some(c) = self.current_char {
					if c.is_ascii_alphanumeric() || c == '_' {
						ident.push(c);
						self.advance();
					} else {
						break;
					}
				}
				Lexer::keyword_or_id(&ident)
			}

			Some(c) if c.is_ascii_digit() => { //handle numeric literals
				let mut num: i64 = 0;
			
				if c == '0' {
					self.advance();
					match self.current_char {
						Some('x') | Some('X') => { //Hexadecimal
							self.advance();
							while let Some(c) = self.current_char {
								if c.is_ascii_hexdigit() {
									num = num * 16
										+ c.to_digit(16).unwrap() as i64;
									self.advance();
								} else {
									break;
								}
							}
						}
						Some(c2) if c2.is_ascii_digit() => { //Octal
							while let Some(c) = self.current_char {
								if c >= '0' && c <= '7' {
									num = num * 8 + (c as i64 - '0' as i64);
									self.advance();
								} else {
									break;
								}
							}
						}
						_ => { //Zero
							num = 0;
						}
					}
				} else { //Decimal
					while let Some(c) = self.current_char {
						if c.is_ascii_digit() {
							num = num * 10 + (c as i64 - '0' as i64);
							self.advance();
						} else {
							break;
						}
					}
				}
				Token::Num(num)
			}

			Some('\'') => {
				self.advance();//skip the opening quote
			
				let c = match self.current_char {
					Some('\\') => {
						self.advance();
						match self.current_char {
							Some('n') => '\n',
							Some('t') => '\t',
							Some('\'') => '\'',
							Some('\"') => '\"',
							Some('\\') => '\\',
							Some(other) => other,
							None => return Token::Unknown('\\'),
						}
					}
					Some(c) => c,
					None => return Token::Unknown('\''),
				};
				self.advance(); //move past the actual char
				//Expect the closing quote
				match self.current_char {
					Some('\'') => {
						self.advance(); //skip closing '
						Token::Num(c as i64)
					}
					_ => Token::Unknown('\''),
				}
			}

			Some('"') => {
				self.advance(); //skip opening quote
				let mut string = String::new();
			
				while let Some(c) = self.current_char {
					if c == '"' {
						self.advance(); //skip closing quote
						return Token::Str(string);
					}
					if c == '\\' {
						self.advance();
						match self.current_char {
							Some('n') => string.push('\n'),
							Some('t') => string.push('\t'),
							Some('r') => string.push('\r'),
							Some('"') => string.push('\"'),
							Some('\'') => string.push('\''),
							Some('\\') => string.push('\\'),
							Some(unknown) => string.push(unknown),
							None => return Token::Unknown('\\'),
						}
					} else {
						string.push(c);
					}
					self.advance();
				}
				//If string wasnt properly closed
				Token::Unknown('"')
			}

			Some(c) => {
				match c {
					'=' => {
						self.advance();
						if self.current_char == Some('=') {
							self.advance();
							Token::Eq
						} else {
							Token::Assign
						}
					}
					'!' => {
						self.advance();
						if self.current_char == Some('=') {
							self.advance();
							Token::Ne
						} else {
							Token::Not
						}
					}
					'<' => {
						self.advance();
						match self.current_char {
							Some('=') => { self.advance(); Token::Le }
							Some('<') => { self.advance(); Token::Shl }
							_ => Token::Lt,
						}
					}
					'>' => {
						self.advance();
						match self.current_char {
							Some('=') => { self.advance(); Token::Ge }
							Some('>') => { self.advance(); Token::Shr }
							_ => Token::Gt,
						}
					}
					'+' => {
						self.advance();
						if self.current_char == Some('+') {
							self.advance();
							Token::Inc
						} else {
							Token::Add
						}
					}
					'-' => {
						self.advance();
						if self.current_char == Some('-') {
							self.advance();
							Token::Dec
						} else {
							Token::Sub
						}
					}
					'&' => {
						self.advance();
						if self.current_char == Some('&') {
							self.advance();
							Token::Lan
						} else {
							Token::And
						}
					}
					'|' => {
						self.advance();
						if self.current_char == Some('|') {
							self.advance();
							Token::Lor
						} else {
							Token::Or
						}
					}
					'*' => { self.advance(); Token::Mul }
					'/' => { self.advance(); Token::Div }
					'%' => { self.advance(); Token::Mod }
					'^' => { self.advance(); Token::Xor }
					'~' => { self.advance(); Token::Tilde }
					'?' => { self.advance(); Token::Cond }
					'[' => { self.advance(); Token::LBracket }
					']' => { self.advance(); Token::RBracket }
					'{' => { self.advance(); Token::LBrace }
					'}' => { self.advance(); Token::RBrace }
					'(' => { self.advance(); Token::LParen }
					')' => { self.advance(); Token::RParen }
					';' => { self.advance(); Token::Semicolon }
					':' => { self.advance(); Token::Colon }
					',' => { self.advance(); Token::Comma }
					'#' => { self.advance(); Token::Hash }
					'\'' => { self.advance(); Token::Quote }
					'"' => { self.advance(); Token::DQuote }
					_ => {
						self.advance();
						Token::Unknown(c)
					}
				}
			}
        }
    }
    
    /// Tokenizes the entire source code
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            if token == Token::EOF {
                tokens.push(Token::EOF);
                break;
            }
            tokens.push(token);
        }

        tokens
    }
}


