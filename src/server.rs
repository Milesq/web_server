use std::convert::Into;
use crate::{
    http_route::HttpMethod,
    response::Response
};

#[derive(Debug)]
pub struct HttpServer {
    routes: Vec<crate::HttpRoute>
}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer {
            routes: Vec::new()
        }
    }

    pub fn get<Responsable>(&mut self, _path: &'static str, _handler: fn() -> Responsable) -> &mut Self
        where Responsable: Into<Response> {
        self.routes.push(crate::HttpRoute {
            method: HttpMethod::GET,
            route: vec!["/".into()]
        });

        self
    }
}