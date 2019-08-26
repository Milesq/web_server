use crate::{HttpCode, HttpMethod, Request, Response};
use std::convert::TryFrom;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{self, TcpListener},
};

type RequestHandler = fn(Request, Response) -> Response;

// first parameter = http path + http method
// second parameter = function handler
#[derive(Debug)]
struct HttpHandler(crate::HttpRoute, RequestHandler);

#[derive(Debug, Default)]
pub struct HttpServer {
    routes: Vec<HttpHandler>,
}

impl HttpServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn route(
        mut self,
        method: HttpMethod,
        path: &'static str,
        handler: RequestHandler,
    ) -> Self {
        self.routes.push(HttpHandler(
            crate::HttpRoute {
                method,
                route: path.to_string(),
            },
            handler,
        ));

        self
    }

    pub fn get(self, path: &'static str, handler: RequestHandler) -> Self {
        self.route(HttpMethod::GET, path, handler)
    }

    pub fn post(self, path: &'static str, handler: RequestHandler) -> Self {
        self.route(HttpMethod::POST, path, handler)
    }
}

impl HttpServer {
    pub fn run(self) -> io::Result<()> {
        self.launch(80)
    }

    pub fn launch(self, port: i32) -> io::Result<()> {
        let ip = if cfg!(debug_assertions) {
            "localhost"
        } else {
            "0.0.0.0"
        };

        let server = TcpListener::bind(format!("{}:{}", ip, port).as_str())?;

        for stream in server.incoming() {
            let mut stream = stream?;
            let received = Request::try_from(read_to_string(&mut stream)?);

            let resp = if received.is_err() {
                Response {
                    response_code: HttpCode::_400,
                    ..Response::new()
                }
            } else if let Ok(mut req) = received {
                let mut resp = Response::new();

                for route in self.routes.iter() {
                    let (routes_matches, params) =
                        matches_to_route(route.0.route.clone(), req.get_path());

                    if route.0.method == req.get_method() && routes_matches {
                        req.params = params;
                        resp = route.1(req, Response::new());
                        break;
                    }
                }

                resp
            } else {
                Response::new()
            };

            stream.write_all(resp.to_string().as_bytes())?;
            stream.shutdown(net::Shutdown::Both)?;
        }

        Ok(())
    }
}

fn read_to_string(readable: &mut impl Read) -> io::Result<String> {
    Ok(String::from_utf8(read_all(readable)?)
        .unwrap()
        .trim_matches(0 as char)
        .to_string())
}

fn read_all(readable: &mut impl Read) -> io::Result<Vec<u8>> {
    const BUF_SIZE: usize = 512;
    let mut total: Vec<u8> = Vec::new();
    let mut buf = [0u8; BUF_SIZE];
    let mut size: usize = BUF_SIZE;

    while size == BUF_SIZE {
        size = readable.read(&mut buf)?;

        for chr in buf.iter() {
            total.push(*chr);
        }

        buf = [0; BUF_SIZE];
    }

    Ok(total)
}

fn matches_to_route(route: String, path: String) -> (bool, HashMap<String, String>) {
    let route = route.split('/').filter(|el| el != &"");
    let path = path.split('/').filter(|el| el != &"");

    if route.clone().count() != path.clone().count() {
        return (false, HashMap::new());
    }

    let mut params: HashMap<String, String> = HashMap::new();

    for el in path.zip(route) {
        let (path, route) = el;
        if &route[..1] != ":" {
            if path != route {
                return (false, HashMap::new());
            }
        } else {
            params.insert(route[1..].to_string(), path.to_string());
        }
    }

    (true, params)
}
