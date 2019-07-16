use crate::ast::{Tag, HTML};
use crate::lexer::Lexer;
use crate::token::*;

#[derive(Clone)]
pub struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let initial_current = Token {
            token_type: None,
            literal: "Hello".to_string(),
        };
        let initial_peek = Token {
            token_type: Some(EOF.to_string()),
            literal: "EOF".to_string(),
        };

        let mut p: Parser = Parser {
            l: l,
            current_token: initial_current,
            peek_token: initial_peek,
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_html(&mut self) -> HTML {
        let html = HTML { tag: Vec::new() };

        loop {
            if let Some(tt) = &self.current_token.token_type {
                if tt == EOF {
                    break;
                }
            }
            self.next_token();
            self.l.read_char();
        }

        html
    }

    fn parse_tag(&mut self) -> Tag {
        let tag = Tag {
            name: "p".to_string(),
            attribute: None,
            text: Some("Hello".to_string()),
        };

        tag
    }
}
