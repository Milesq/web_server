#[derive(Debug)]
/// Storing http methods and path
pub struct HttpRoute {
    pub method: HttpMethod,
    pub route: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
/// Represents http's methods
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    OPTIONS,
    Any,
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
