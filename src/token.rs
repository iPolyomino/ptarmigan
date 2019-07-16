pub type TokenType = String;

pub const EOF: &str = "EOF";

pub const IDENT: &str = "IDENT";

pub const LT: &str = "<";
pub const GT: &str = ">";
pub const SLASH: &str = "/";

pub const TEXT: &str = "TEXT";

#[derive(Clone)]
pub struct Token {
    pub token_type: Option<TokenType>,
    pub literal: String,
}
