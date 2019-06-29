use crate::lexer::Lexer;

pub struct Parser {
    l: Lexer,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        Parser { l: l }
    }
}
