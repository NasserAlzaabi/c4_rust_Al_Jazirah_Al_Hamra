use c4::lexer::{Lexer, Token};

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
