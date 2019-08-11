pub mod http_path;
mod http_method;

pub use http_path::*;
pub use http_method::HttpMethod;

#[derive(Debug)]
pub struct HttpRoute {
    pub method: HttpMethod,
    pub route: HttpPath
}
