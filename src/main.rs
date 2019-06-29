extern crate ptarmigan;

use ptarmigan::lexer::Lexer;
use ptarmigan::parser::Parser;

const SAMPLE_HTML: &'static str = "
<p>
  hello world
</p>
";

fn main() {
    let l: Lexer = Lexer::new(SAMPLE_HTML.to_string());
    let mut p: Parser = Parser::new(l);
    p.parse_html();
}
