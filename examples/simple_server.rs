extern crate http_server;

// struct Respons
// impl Into<Responsable> for ()
// impl Into<Responsable> for http_sever::Response
// impl Into<Responsable> for Result<(), std::io::Error>
// fn handler(req: http_server::Request, resp: http_server::Response) -> std::io::Result<()> {
//     write!(resp, "ok")?;
//     Ok(())
// }

fn main() {
    // http_server::new()
    //     .get("/", |_, _| "ok")
    //     .launch(8080);
    // let mut x = http_server::new();

    // x
    //     .get("/stats/:num", || "ok")
    //     .post("/stats/:num/s", || "okadf");
    let mut x = http_server::HttpServer::new()
        .get("/stats/:num", || "ok")
        .post("/stats/:num/s", || "okadf");

    println!("{:#?}", x);
}