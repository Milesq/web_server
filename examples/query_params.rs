extern crate web_server;

fn main() {
    web_server::new()
        .get("/user/:id", |req, _| format!("{:#?}", req).into())
        .launch(8080)
        .unwrap();
}
