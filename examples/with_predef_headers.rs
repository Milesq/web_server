use std::collections::HashMap;
use web_server::{route, Response};

fn main() {
    let mut headers = HashMap::new();
    headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());

    let default_response = Response {
        headers,
        .."".into()
    };

    web_server::create_server(default_response)
        .get(
            "/",
            &route!(|_, _| "It can be fetched from browser by AJAX!".into()),
        )
        .launch(8080);
}
