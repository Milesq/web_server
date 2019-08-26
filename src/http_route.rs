#[derive(Debug)]
pub struct HttpRoute {
    pub method: HttpMethod,
    pub route: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    OPTIONS,
}
