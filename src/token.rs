#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    IDENT,
    LT,
    GT,
    SLASH,
    TEXT,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub literal: String,
}
