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

#[derive(Debug, Clone, PartialEq, Eq)]


//Token types as enum
pub enum Token {
	Num(i64),	//Numeric literal
	Id(String),	//Identifier
	Str(String), //String literals

	//Keywords
	Char, Else, Enum, If, Int, Return, Sizeof, While,

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
	Brak,    // [

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

	Unknown(char), //unkown character
}

//Lexer struct
pub struct Lexer<'a> {
	pub source: &'a str, //original source code
	pub chars: std::str::Chars<'a>, //iterator over the source
	pub current_char: Option<char>, //current character
	pub line: usize, //current line number
	pub peeked: Option<char>, //one-character lookahead
}

//Lexer struct implementation
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let current_char = chars.next();

        Lexer {
            source,
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
			_ => Token::Id(ident.to_string()),
		}
	}

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

			//catch all fail safe
            Some(c) => {
                self.advance();
                Token::Unknown(c)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("   \n\t   \n  ");
        let token = lexer.next_token();
        assert_eq!(token, Token::EOF);
        assert_eq!(lexer.line, 3); // 2 newline advances
    }

    #[test]
    fn test_skip_hash_comment() {
        let mut lexer = Lexer::new("# this is a comment\n\n");
        let token = lexer.next_token();
        assert_eq!(token, Token::EOF);
        assert_eq!(lexer.line, 3);
    }

    #[test]
    fn test_skip_double_slash_comment() {
        let mut lexer = Lexer::new("// comment one\n// comment two\n");
        let token = lexer.next_token();
        assert_eq!(token, Token::EOF);
        assert_eq!(lexer.line, 3);
    }

    #[test]
    fn test_mixed_whitespace_and_comments() {
        let mut lexer = Lexer::new("   // hello\n  # again\n\n\t\t");
        let token = lexer.next_token();
        assert_eq!(token, Token::EOF);
        assert_eq!(lexer.line, 4);
    }

	#[test]
	fn test_identifier_basic() {
	    let mut lexer = Lexer::new("hello");
	    let token = lexer.next_token();
	    assert_eq!(token, Token::Id("hello".to_string()));
	}

	#[test]
	fn test_identifier_with_numbers() {
	    let mut lexer = Lexer::new("var123_name");
	    let token = lexer.next_token();
	    assert_eq!(token, Token::Id("var123_name".to_string()));
	}

	#[test]
	fn test_keywords() {
	    let keywords = vec![
	        ("char", Token::Char),
	        ("else", Token::Else),
	        ("enum", Token::Enum),
	        ("if", Token::If),
	        ("int", Token::Int),
	        ("return", Token::Return),
	        ("sizeof", Token::Sizeof),
	        ("while", Token::While),
	    ];

	    for (word, expected_token) in keywords {
	        let mut lexer = Lexer::new(word);
	        let token = lexer.next_token();
	        assert_eq!(token, expected_token);
	    }
	}

	#[test]
	fn test_decimal_number() {
	    let mut lexer = Lexer::new("1234");
	    let token = lexer.next_token();
	    assert_eq!(token, Token::Num(1234));
	}

	#[test]
	fn test_zero() {
		let mut lexer = Lexer::new("0");
		let token = lexer.next_token();
		assert_eq!(token, Token::Num(0));
	}

	#[test]
	fn test_octal_number() {
		let mut lexer = Lexer::new("077");
		let token = lexer.next_token();
		assert_eq!(token, Token::Num(63)); // 7*8 + 7 = 63
	}

	#[test]
	fn test_hex_number() {
		let mut lexer = Lexer::new("0x1A3F");
		let token = lexer.next_token();
		assert_eq!(token, Token::Num(0x1A3F));
	}

	#[test]
	fn test_char_literal_simple() {
		let mut lexer = Lexer::new("'a'");
		let token = lexer.next_token();
		assert_eq!(token, Token::Num('a' as i64));
	}
	
	#[test]
	fn test_char_literal_escape_newline() {
		let mut lexer = Lexer::new("'\\n'");
		let token = lexer.next_token();
		assert_eq!(token, Token::Num('\n' as i64));
	}
	
	#[test]
	fn test_char_literal_escape_single_quote() {
		let mut lexer = Lexer::new("'\\''");
		let token = lexer.next_token();
		assert_eq!(token, Token::Num('\'' as i64));
	}
	
	#[test]
	fn test_char_literal_backslash() {
		let mut lexer = Lexer::new("'\\\\'");
		let token = lexer.next_token();
		assert_eq!(token, Token::Num('\\' as i64));
	}

	#[test]
	fn test_string_literal_basic() {
		let mut lexer = Lexer::new("\"hello\"");
		let token = lexer.next_token();
		assert_eq!(token, Token::Str("hello".to_string()));
	}
	
	#[test]
	fn test_string_literal_with_escape() {
		let mut lexer = Lexer::new("\"line\\nbreak\"");
		let token = lexer.next_token();
		assert_eq!(token, Token::Str("line\nbreak".to_string()));
	}
	
	#[test]
	fn test_string_literal_with_quote_and_backslash() {
		let mut lexer = Lexer::new("\"a \\\"quote\\\" and \\\\ backslash\"");
		let token = lexer.next_token();
		assert_eq!(token, Token::Str("a \"quote\" and \\ backslash".to_string()));
	}
	
	#[test]
	fn test_unterminated_string_literal() {
		let mut lexer = Lexer::new("\"unfinished");
		let token = lexer.next_token();
		assert_eq!(token, Token::Unknown('"'));
	}

	#[test]
	fn test_single_char_operators() {
		let symbols = vec![
			("+", Token::Add),
			("-", Token::Sub),
			("*", Token::Mul),
			("/", Token::Div),
			("%", Token::Mod),
			("&", Token::And),
			("|", Token::Or),
			("^", Token::Xor),
			("!", Token::Not),
			("~", Token::Tilde),
			("?", Token::Cond),
			("=", Token::Assign),
			("<", Token::Lt),
			(">", Token::Gt),
			(";", Token::Semicolon),
			(":", Token::Colon),
			(",", Token::Comma),
			("(", Token::LParen),
			(")", Token::RParen),
			("{", Token::LBrace),
			("}", Token::RBrace),
			("[", Token::LBracket),
			("]", Token::RBracket),
		];
	
		for (input, expected) in symbols {
			let mut lexer = Lexer::new(input);
			assert_eq!(lexer.next_token(), expected);
		}
	}
	
	#[test]
	fn test_double_char_operators() {
		let symbols = vec![
			("==", Token::Eq),
			("!=", Token::Ne),
			("<=", Token::Le),
			(">=", Token::Ge),
			("++", Token::Inc),
			("--", Token::Dec),
			("&&", Token::Lan),
			("||", Token::Lor),
			("<<", Token::Shl),
			(">>", Token::Shr),
		];
	
		for (input, expected) in symbols {
			let mut lexer = Lexer::new(input);
			assert_eq!(lexer.next_token(), expected);
		}
	}
	
	#[test]
	fn test_unknown_tokens() {
	    let mut lexer = Lexer::new("ض");
	    let token = lexer.next_token();
	    assert_eq!(token, Token::Unknown('ض'));
	
	    let mut lexer = Lexer::new("س");
	    let token = lexer.next_token();
	    assert_eq!(token, Token::Unknown('س'));
	}
}
