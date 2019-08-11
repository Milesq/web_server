use crate::http_route::HttpMethod;

#[derive(Debug)]
pub struct HttpServer {
    pub routes: Vec<crate::HttpRoute>
}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer {
            routes: Vec::new()
        }
    }

    pub fn get(&mut self, _path: &str, _handler: fn() -> ()) -> &mut Self {
        self.routes.push(crate::HttpRoute {
            method: HttpMethod::GET,
            route: vec!["/".into()]
        });
        self
    }
}