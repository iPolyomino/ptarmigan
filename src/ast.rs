#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    pub name: String,
    pub attribute: Option<String>,
    pub texts: Vec<String>,
    pub child: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HTML {
    pub tag: Vec<Tag>,
}
