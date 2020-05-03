extern crate web_server;

fn main() {
    web_server::new()
        .get("/user/:id", |req, _| format!("{:#?}", req.query).into())
        .launch(8080);
}
