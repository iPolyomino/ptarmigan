#[macro_use]
extern crate html5ever;
extern crate reqwest;

use std::default::Default;
use std::iter::repeat;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

static URL: &str = "https://ipolyomino.github.io/simple-html/";

fn walk(mut indent: usize, handle: Handle) {
    let node = handle;
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document => {
            println!("#Document");
            indent += 4;
        },
        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),
        NodeData::Text { ref contents } => {
            if !contents.borrow().contains("\n") {
                println!("#Text: {}", escape_default(&contents.borrow()))
            }
        }
        NodeData::Comment { ref contents } => println!("<!-- {} -->", escape_default(contents)),
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }
        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }
    for child in node.children.borrow().iter() {
        walk(indent, child.clone());
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn main() {
    let mut response = reqwest::get(URL).unwrap();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut response)
        .unwrap();
    if !dom.errors.is_empty() {
        eprintln!("parse error");
        for err in dom.errors.into_iter() {
            eprintln!("{}", err);
        }
        return;
    }

    let handle = dom.document;
    let indent = 0;
    walk(indent, handle);
}
