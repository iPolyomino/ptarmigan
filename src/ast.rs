#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    pub name: String,
    pub attribute: Option<String>,
    pub texts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HTML {
    pub tag: Vec<Tag>,
}
