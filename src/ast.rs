pub struct Tag {
    pub name: String,
    pub attribute: Option<String>,
    pub text: Option<String>,
}

pub struct HTML {
    pub tag: Vec<Tag>,
}
