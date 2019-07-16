use crate::token::*;

#[derive(Clone)]
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

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or(' ');
            self.read_position += 1;
        }
    }

    pub fn next_token(&mut self) -> Token {
        println!("{}", self.ch);

        let mut tok: Token;

        match self.ch {
            '/' => {
                tok = Token {
                    token_type: Some(SLASH.to_string()),
                    literal: self.ch.to_string(),
                }
            }
            'p' => {
                tok = Token {
                    token_type: Some(IDENT.to_string()),
                    literal: self.ch.to_string(),
                }
            }
            '0' => {
                tok = Token {
                    token_type: Some(EOF.to_string()),
                    literal: "".to_string(),
                }
            }
            _ => {
                tok = Token {
                    token_type: Some(TEXT.to_string()),
                    literal: self.ch.to_string(),
                }
            }
        }

        tok
    }
}
