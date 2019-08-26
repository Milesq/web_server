mod http_route;
mod http_version;
mod response;
mod server;
// mod request;
mod http_code;

pub use http_code::HttpCode;
pub use http_route::{HttpMethod, HttpRoute};
use http_version::HttpVersion;
pub use response::Response;
// pub use request::Request;
pub use server::HttpServer;

pub fn new() -> HttpServer {
    HttpServer::new()
}
