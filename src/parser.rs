use crate::ast::{Tag, HTML};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let initial_current = Token {
            token_type: None,
            literal: " ".to_string(),
        };
        let initial_peek = Token {
            token_type: None,
            literal: " ".to_string(),
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
        let mut html = HTML { tag: Vec::new() };

        while let Some(tt) = &self.current_token.token_type {
            match tt {
                TokenType::IDENT => self.next_token(),
                TokenType::LT => {
                    let tag: Tag = self.parse_tag();
                    html.tag.push(tag);
                    self.next_token();
                }
                TokenType::GT => self.next_token(),
                TokenType::SLASH => self.next_token(),
                TokenType::TEXT => self.next_token(),
                TokenType::EOF => break,
            }
        }

        html
    }

    fn parse_tag(&mut self) -> Tag {
        // read LT
        self.next_token();

        let mut tag_name = "".to_string();

        while self.current_token.token_type.as_ref().unwrap() == &TokenType::IDENT {
            tag_name = [tag_name, self.current_token.literal.clone()].concat();
            self.next_token();
        }

        if self.current_token.token_type.as_ref().unwrap() == &TokenType::SLASH {
            // read SLASH
            self.next_token();
        }

        // read GT
        self.next_token();

        let mut text = "".to_string();
        while self.current_token.token_type.as_ref().unwrap() == &TokenType::TEXT {
            text = [text, self.current_token.literal.clone()].concat();
            self.next_token();
        }

        let tag = Tag {
            name: tag_name,
            attribute: None,
            text: Some(text),
        };

        tag
    }
}
