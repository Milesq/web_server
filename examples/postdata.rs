extern crate web_server;

fn main() {
    web_server::new()
        .post("/add-user", |req, _| {
            format!("{:#?}", req.get_body()).into()
        })
        .launch(8080)
        .unwrap();
}
