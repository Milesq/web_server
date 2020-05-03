use crate::{HttpCode, HttpMethod, Request, Response};
use std::convert::TryFrom;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{self, TcpListener},
    thread,
    sync::Arc,
};

type RequestHandler = fn(Request, Response) -> Response;

// first parameter = http path + http method
// second parameter = function handler
#[derive(Debug)]
struct HttpHandler(crate::HttpRoute, RequestHandler);

#[derive(Debug)]
/// Storing basics informations about server and handlers
/// Represents http server
pub struct HttpServer {
    routes: Vec<HttpHandler>,
    not_found_handler: RequestHandler,
    pub(crate) default_repsonse: Response,
}

impl Default for HttpServer {
    fn default() -> Self {
        Self {
            routes: Vec::new(),
            not_found_handler: |_, mut default_resp| {
                default_resp.response_code = HttpCode::_404;
                default_resp
            },
            default_repsonse: Response::new(),
        }
    }
}

impl HttpServer {
    /// Create new instance of HttpServer
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a route for a specific path and method
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

    /// Add a route for GET method and a specific path
    /// ```
    /// server.get("/user/:id", |request, _| {
    ///     database.get_user(request.params.get("id").unwrap()).into()
    /// })
    /// ```
    pub fn get(self, path: &'static str, handler: RequestHandler) -> Self {
        self.route(HttpMethod::GET, path, handler)
    }

    /// Add a route for POST method and a specific path
    /// ```
    /// server.post("/add-user", |request, mut default_response| {
    ///     println!("Save new user!\n\n{}", request.get_body());
    ///
    ///     default_response.headers.insert("Access-Control-Allow-Origin", "*");
    ///     default_repsonse
    /// })
    /// ```
    pub fn post(self, path: &'static str, handler: RequestHandler) -> Self {
        self.route(HttpMethod::POST, path, handler)
    }

    /// Add a route for a specific path and any method
    /// ```
    /// server
    ///     .get("/endpoint", |request, default_response| "Gate GET were obtained".into())
    ///     .post("/endpoint", |request, default_response| "Gate POST were obtained".into())
    ///     .any("/endpoint", |request, default_response| "Another gate were obtained".into())
    /// ```
    pub fn any(self, path: &'static str, handler: RequestHandler) -> Self {
        self.route(HttpMethod::Any, path, handler)
    }

    /// Add a handler for 404 error
    /// ```
    /// server.not_found(|_, _| "Not found!".into());
    /// ```
    pub fn not_found(mut self, handler: RequestHandler) -> Self {
        self.not_found_handler = handler;
        self
    }
}

impl HttpServer {
    /// Launch server on port 80
    pub fn run(self) -> ! {
        self.launch(80)
    }

    /// Lauch http server, never returns

    /// ```
    /// server.launch(8080).unwrap();
    /// ```
    pub fn launch(self, port: i32) -> ! {
        let ip = if cfg!(debug_assertions) {
            "localhost"
        } else {
            "0.0.0.0"
        };

        let server = TcpListener::bind(format!("{}:{}", ip, port).as_str()).unwrap();
        let this = Arc::new(self);

        for stream in server.incoming() {
            let this = Arc::clone(&this);

            thread::spawn(move || {
                if let Ok(mut stream) = stream {
                    let received = Request::try_from(read_to_string(&mut stream).unwrap_or_default());

                    let resp = match received {
                        Err(_) => Response {
                            response_code: HttpCode::_400,
                            ..Response::new()
                        },
                        Ok(mut req) => {
                            let mut resp = this.default_repsonse.clone();
                            let mut matched_route: bool = false;

                            for route in this.routes.iter() {
                                let (routes_matches, params) =
                                    matches_to_route(route.0.route.clone(), req.get_path());

                                if (route.0.method == req.get_method()
                                    || route.0.method == HttpMethod::Any)
                                    && routes_matches
                                {
                                    req.params = params;
                                    resp = route.1(req.clone(), Response::new());
                                    matched_route = true;
                                    break;
                                }
                            }

                            if !matched_route {
                                resp = (this.not_found_handler)(req.clone(), Response::new());
                            }

                            resp
                        }
                    };

                    stream
                        .write_all(resp.to_string().as_bytes())
                        .unwrap_or_else(|_| println!("Cannot write packet"));
                    stream
                        .shutdown(net::Shutdown::Both)
                        .unwrap_or_else(|_| println!("Cannot shutdown connection"));
                }
            });
        }

        loop {}
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
