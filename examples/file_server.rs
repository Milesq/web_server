use std::path::Path;
use web_server::route;

fn main() {
    web_server::new()
        .get("/", &route!(|_, _| Path::new("examples/sw.png").into()))
        .launch(8080);
}
