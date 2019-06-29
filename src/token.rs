pub struct Tag {
    element: &'static str,
    attribute: &'static str,
}

pub struct HyperText {
    pub tag: Option<Tag>,
    pub text: &'static str,
}
