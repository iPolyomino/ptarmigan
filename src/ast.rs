pub struct Statement {}

impl Statement {
    pub fn statement_node() {}
}

pub trait Node {
    fn token_literal() -> String;
    fn string() -> String;
}

impl Node for Statement {
    fn token_literal() -> String {
        "Hello".to_string()
    }
    fn string() -> String {
        "Hi".to_string()
    }
}

pub struct HTML {
    pub statements: Vec<Statement>,
}
