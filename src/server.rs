use crate::{
    http_route::HttpMethod,
    response::Response
};
use std::convert::Into;

// type HttpListener = &dyn Fn() -> ;

#[derive(Debug)]
struct HttpHandler(crate::HttpRoute, i32);

#[derive(Debug)]
pub struct HttpServer {
    routes: Vec<HttpHandler>
}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer { routes: Vec::new() }
    }

    pub fn route<Responsable>(
        mut self,
        method: HttpMethod,
        path: &'static str,
        _handler: fn() -> Responsable,
    ) -> Self
    where Responsable: Into<Response> {
        self.routes.push(HttpHandler(crate::HttpRoute {
            method: method,
            route: path.to_string()
        }, 7));

        self
    }

    pub fn get<Responsable>(
        self,
        path: &'static str,
        _handler: fn() -> Responsable,
    ) -> Self
    where Responsable: Into<Response> {
        self.route(HttpMethod::GET, path, _handler)
    }

    pub fn post<Responsable>(
        self,
        path: &'static str,
        _handler: fn() -> Responsable,
    ) -> Self
    where Responsable: Into<Response> {
        self.route(HttpMethod::POST, path, _handler)
    }
}
