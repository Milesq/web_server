mod server;
mod http_route;
mod response;

pub use server::HttpServer;
pub use http_route::{HttpRoute, HttpMethod};
pub use response::Response;

pub fn new() -> HttpServer {
    HttpServer::new()
}
