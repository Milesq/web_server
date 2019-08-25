mod http_version;
mod server;
mod http_route;
mod response;
mod request;

use http_version::HttpVersion;
pub use server::HttpServer;
pub use http_route::{HttpRoute, HttpMethod};
pub use response::Response;
pub use request::Request;

pub fn new() -> HttpServer {
    HttpServer::new()
}
