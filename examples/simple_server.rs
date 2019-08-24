extern crate http_server;

// fn handler(req: http_server::Request, resp: http_server::Response) -> impl http_server::Response {

fn main() {
    // http_server::new()
    //     .get("/", |_, _| "ok")
    //     .launch(8080);

    let x = http_server::new()
        .get("/stats/:num", || "ok".into());

    println!("{:#?}", x);
}