use crate::ast::HTML;
use crate::lexer::Lexer;
use crate::token::*;

pub struct Parser {
    l: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let initial_token = Token {
            token_type: None,
            literal: "Hello".to_string(),
        };

        Parser {
            l: l,
            current_token: initial_token,
        }
    }

    pub fn parse_html(&mut self) -> HTML {
        let html = HTML {
            statements: Vec::new(),
        };

        loop {
            if let Some(tt) = &self.current_token.token_type {
                if tt == EOF {
                    break;
                }
            }
            self.current_token.token_type = Some(EOF.to_string());
            self.l.read_char();
        }

        html
    }
}
