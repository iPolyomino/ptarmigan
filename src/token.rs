pub type TokenType = String;

pub const EOF: &str = "EOF";

pub const IDENT: &str = "IDENT";

pub const LT: &str = "LT";
pub const GT: &str = "GT";
pub const SLASH: &str = "SLASH";

pub const TEXT: &str = "TEXT";

#[derive(Clone)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub literal: String,
}
