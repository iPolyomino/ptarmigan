extern crate ptarmigan;

use ptarmigan::parser::Parser;
use ptarmigan::lexer::Lexer;

const SAMPLE_HTML: &'static str = "
<p>
  hello world
</p>
";

fn main() {
    let l : Lexer = Lexer::new(SAMPLE_HTML.to_string());
    let _ : Parser = Parser::new(l);
}
