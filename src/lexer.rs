use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lexer {
    pub input: String,
    pub position: usize,      // current position
    pub read_position: usize, // next position
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 1,
            ch: ' ',
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_white_space();

        match self.ch {
            '<' => {
                tok = Token {
                    token_type: Some(TokenType::LT),
                    literal: self.ch.to_string(),
                }
            }
            '>' => {
                tok = Token {
                    token_type: Some(TokenType::GT),
                    literal: self.ch.to_string(),
                }
            }
            '/' => {
                tok = Token {
                    token_type: Some(TokenType::SLASH),
                    literal: self.ch.to_string(),
                }
            }
            'p' => {
                tok = Token {
                    token_type: Some(TokenType::IDENT),
                    literal: self.ch.to_string(),
                }
            }
            '\0' => {
                tok = Token {
                    token_type: Some(TokenType::EOF),
                    literal: "".to_string(),
                }
            }
            _ => {
                tok = Token {
                    token_type: Some(TokenType::TEXT),
                    literal: self.ch.to_string(),
                }
            }
        }

        self.read_char();

        tok
    }

    fn skip_white_space(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or(' ');
            self.read_position += 1;
        }
    }
}

#[test]
fn test_lexer() {
    const SAMPLE_HTML: &'static str = "
<p>
  hello world
</p>
";
    let lexer: Lexer = Lexer::new(SAMPLE_HTML.to_string());

    assert_eq!(
        lexer,
        Lexer {
            input: "\n<p>\n  hello world\n</p>\n".to_string(),
            position: 0,
            read_position: 2,
            ch: '<'
        }
    );
}
