extern crate http_server;

use http_server::Response;
use std::collections::HashMap;

fn main() {
    let mut headers = HashMap::new();
    headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());

    let default_response = Response {
        headers,
        .."".into()
    };

    http_server::create_server(default_response)
        .get("/", |_, _| "It can be fetched from browser by AJAX!".into())
        .launch(8080)
        .unwrap();
}
