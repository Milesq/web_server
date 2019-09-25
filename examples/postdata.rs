extern crate web_server;

use std::collections::HashMap;
use web_server::decoders::x_www_form_urlencoded;

fn main() {
    web_server::new()
        .post("/add-user", |req, _| {
            println!("{}", req.get_body());
            let body: HashMap<String, String> = x_www_form_urlencoded(req.get_body().as_str());
            format!(
                "Add new user: {}",
                body.get("user").unwrap_or(&String::from("Error!"))
            )
            .into()
        })
        .launch(8080)
        .unwrap();
}
