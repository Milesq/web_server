mod http_code;
mod http_route;
mod http_version;
mod request;
mod response;
mod server;

pub use http_code::HttpCode;
pub use http_route::{HttpMethod, HttpRoute};
use http_version::HttpVersion;
pub use request::Request;
pub use response::Response;
pub use server::HttpServer;

/// Create new instance of HttpServer
pub fn new() -> HttpServer {
    HttpServer::new()
}
