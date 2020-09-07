extern crate ptarmigan;

use ptarmigan::lexer::Lexer;
use ptarmigan::parser::Parser;

const SAMPLE_HTML: &'static str = "
<p>
  <span>hello world</span>
</p>
";

fn main() {
    let l: Lexer = Lexer::new(SAMPLE_HTML.to_string());
    let mut p: Parser = Parser::new(l);
    let ast = p.parse_html();
    println!("{:?}", ast);
}
