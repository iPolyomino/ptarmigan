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
            l: l.to_owned(),
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
                TokenType::IDENT => {
                    let tag: Tag = self.parse_tag();
                    html.tag.push(tag);
                    self.next_token();
                }
                TokenType::LT => self.next_token(),
                TokenType::GT => self.next_token(),
                TokenType::SLASH => {
                    if self.peek_token.token_type == Some(TokenType::IDENT) {
                        // close tag
                        self.next_token();
                    }
                    self.next_token();
                }
                TokenType::TEXT => {
                    if let Some(last) = &html.tag.pop() {
                        let mut last_clone = last.clone();
                        last_clone
                            .texts
                            .push(self.current_token.literal.to_string());
                        html.tag.push(last_clone);
                    }
                    self.next_token();
                }
                TokenType::EOF => break,
            }
        }

        html
    }

    fn parse_tag(&mut self) -> Tag {
        let tag = Tag {
            name: self.current_token.literal.clone(),
            attribute: None,
            texts: vec![],
        };

        tag
    }
}

#[test]
fn test_parser() {
    const SAMPLE_HTML: &'static str = "
<p>
  <span>hello world</span>
</p>
";
    let l: Lexer = Lexer::new(SAMPLE_HTML.to_string());
    let mut p: Parser = Parser::new(l);
    let ast = p.parse_html();

    assert_eq!(
        ast,
        HTML {
            tag: vec![
                Tag {
                    name: "p".to_string(),
                    attribute: None,
                    texts: vec![]
                },
                Tag {
                    name: "span".to_string(),
                    attribute: None,
                    texts: vec!["hello".to_string(), "world".to_string()]
                }
            ]
        }
    );
}
