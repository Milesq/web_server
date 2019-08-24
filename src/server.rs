use crate::{
    http_route::HttpMethod,
    response::Response
};

type RequestHandler = fn() -> Response;

// first parameter = http path + http method
// second parameter = function handler
#[derive(Debug)]
struct HttpHandler(crate::HttpRoute, RequestHandler);

#[derive(Debug)]
pub struct HttpServer {
    routes: Vec<HttpHandler>
}

impl<'a> HttpServer {
    pub fn new() -> HttpServer {
        HttpServer { routes: Vec::new() }
    }

    pub fn route<'b>(
        mut self,
        method: HttpMethod,
        path: &'static str,
        handler: RequestHandler,
    ) -> Self {
        self.routes.push(HttpHandler(crate::HttpRoute {
            method: method,
            route: path.to_string()
        }, handler));

        self
    }

    pub fn get(
        self,
        path: &'static str,
        handler: RequestHandler,
    ) -> Self {
        self.route(HttpMethod::GET, path, handler)
    }

    pub fn post(
        self,
        path: &'static str,
        handler: RequestHandler,
    ) -> Self {
        self.route(HttpMethod::POST, path, handler)
    }
}
