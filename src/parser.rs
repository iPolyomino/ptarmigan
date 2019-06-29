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
            token_type: EOF.to_string(),
            literal: "Hello".to_string(),
        };

        Parser {
            l: l,
            current_token: initial_token,
        }
    }

    pub fn parse_html(&mut self) -> HTML {
        let mut html = HTML {
            statements: Vec::new(),
        };

        println!("{}", self.current_token.token_type);
        while self.current_token.token_type != EOF {
            self.current_token.token_type = EOF.to_string();
            println!("{}", self.current_token.token_type);
            self.l.read_char();
        }

        html
    }
}
