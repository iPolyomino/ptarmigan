pub struct Lexer {
    pub input: String,
    pub position: usize,      // current position
    pub read_position: usize, // next position
    pub ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 1,
            ch: None,
        };
        l.read_char();
        l
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
            self.read_position += 1;
        }
    }
}
