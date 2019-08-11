pub type HttpPath = Vec<HttpPathPart>;

#[derive(Debug)]
pub enum HttpPathPart {
    Identifier(String),
    Var(String)
}

pub fn split_path(path: String) -> HttpPath {
    vec![HttpPathPart::Identifier("/".into())]
}