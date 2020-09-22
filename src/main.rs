extern crate ptarmigan;

use ptarmigan::lexer::Lexer;
use ptarmigan::parser::Parser;

const SAMPLE_HTML: &'static str = "
<html>
  <span>hello world</span>
  <p>this is test.</p>
</html>
";

fn main() {
    let l: Lexer = Lexer::new(SAMPLE_HTML.to_string());
    let mut p: Parser = Parser::new(l);
    let ast = p.parse_html();
    println!("{:?}", ast);
}
