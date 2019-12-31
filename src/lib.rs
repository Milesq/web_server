//! web_server is a small, dependency-less crate for creating HTTP servers.
//! When you coding the backend using Rust,
//! the most annoying thing could be the size of a freamwork
//! and the time needed to compile the application
//!
//! The web_server package fixes these problems.
//! web_server has no dependencies, but allows you to create full-fledged servers
//!
//!
//! First server using web_server
//! ```
//! extern crate web_server;
//! web_server::new()
//!    .get("/", |request: web_server::Request, mut response: web_server::Response|
//!         "Hello World!".into())
//!    .launch(80)
//!    .unwrap();
//! ```
//!
//! It's easy!
//! First you must create instance of HttpServer
//! ```
//! web_server::new()
//! ```
//! then you can declare your endpoints. E.g.
//! ```
//! .get("/your/path", |request, default_response| {
//!     // There place your logic
//!     // This function returns Response
//!     "response text".into()
//! })
//! .post("/your/path", |_, _| "Handler for POST method")
//! .route(web_server::HttpMethod::DELETE, "/your/path", |_, _| "Handler for DELETE method")
//! .any("/your/path", |_, _| "Handler for any method")
//! ```
//!
//! Now you must run server by launch method
//! ```
//! .launch(PORT).unwrap()
//! ```
//!
//! You can send files to client e.g.
//! ```
//! .get("/image.png", |_, _| {
//!     std::path::Path::new("path to your file").into();
//! })
//! ```
mod http_code;
mod http_route;
mod http_version;
mod request;
mod response;
mod server;

pub mod decoders {
    pub use crate::request::x_www_form_urlencoded;
}

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

// Create new instance of HttpServer with predefined body
pub fn create_server(default_repsonse: Response) -> HttpServer {
    let mut ret = HttpServer::new();
    ret.default_repsonse = default_repsonse;
    ret
}
