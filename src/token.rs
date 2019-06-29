type TokenType = String;

pub const EOF: &str = "EOF";
pub const INT: &str = "INT";

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub struct Tag {
    element: &'static str,
    attribute: &'static str,
}

pub struct HyperText {
    pub tag: Option<Tag>,
    pub text: &'static str,
}
