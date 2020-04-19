#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    EOF,
    IDENT,
    LT,
    GT,
    SLASH,
    TEXT,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub literal: String,
}
