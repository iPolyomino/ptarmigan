pub type TokenType = String;

pub const EOF: &str = "EOF";

pub const LT: &str = "<";
pub const GT: &str = ">";

pub const SLASH: &str = "/";

#[derive(Clone)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub literal: String,
}
