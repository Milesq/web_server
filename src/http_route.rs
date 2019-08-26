#[derive(Debug)]
pub struct HttpRoute {
    pub method: HttpMethod,
    pub route: String,
}

#[derive(Debug, Clone, Copy)]
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

impl std::str::FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "TRACE" => Ok(HttpMethod::TRACE),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(()),
        }
    }
}
