use crate::token::HyperText;

pub fn parse_html(html: &'static str) -> HyperText {
    HyperText {
        tag: None,
        text: html,
    }
}
