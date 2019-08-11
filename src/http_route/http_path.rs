use std::convert::From;

pub type HttpPath = Vec<HttpPathPart>;

#[derive(Debug)]
pub enum HttpPathPart {
    Identifier(String),
    Var(String)
}

impl From<&str> for HttpPathPart {
    fn from(_path: &str) -> Self {
        HttpPathPart::Identifier("/".into())
    }
}
