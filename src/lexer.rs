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

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token;

        self.skip_white_space();

        match self.ch {
            '<' => {
                tok = Token {
                    token_type: Some(LT.to_string()),
                    literal: self.ch.to_string(),
                }
            }
            '>' => {
                tok = Token {
                    token_type: Some(GT.to_string()),
                    literal: self.ch.to_string(),
                }
            }
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
            '\0' => {
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

    fn new_token(self, token_type: TokenType, ch: String) -> Token {
        Token {
            token_type: Some(token_type),
            literal: ch,
        }
    }
}
