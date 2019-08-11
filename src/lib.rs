mod server;
mod http_route;

pub use server::HttpServer;
pub use http_route::HttpRoute;

pub fn new() -> HttpServer {
    HttpServer::new()
}
