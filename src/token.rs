type TokenType = String;

pub const EOF: &str = "EOF";
pub const INT: &str = "INT";

pub struct Token {
    pub token_type: Option<TokenType>,
    pub literal: String,
}
