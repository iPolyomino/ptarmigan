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
                self.read_char();
                let mut buf = "".to_string();
                while self.ch != '>' {
                    buf += &self.ch.to_string();
                    self.read_char();
                }
                self.read_char();

                tok = Token {
                    token_type: Some(TokenType::IDENT),
                    literal: buf + &self.ch.to_string(),
                };
            }
            '\0' => {
                tok = Token {
                    token_type: Some(TokenType::EOF),
                    literal: "".to_string(),
                };
                self.read_char();
            }
            _ => {
                let mut buf = self.ch.to_string();
                self.read_char();
                while self.ch != '<' && self.ch != '\0' {
                    buf += &self.ch.to_string();
                    self.read_char();
                }

                tok = Token {
                    token_type: Some(TokenType::TEXT),
                    literal: buf,
                };
            }
        }

        println!("{}", tok.literal);
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
