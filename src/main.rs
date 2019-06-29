extern crate ptarmigan;

use ptarmigan::parser::parse_html;
use ptarmigan::token::HyperText;

const SAMPLE_HTML: &'static str = "hello world";

fn main() {
    let ht: HyperText = parse_html(SAMPLE_HTML);
    println!("{}", ht.text);
}
