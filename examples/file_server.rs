use std::path::Path;

fn main() {
    web_server::new()
        .get("/", Box::new(|_, _| Path::new("examples/sw.png").into()))
        .launch(8080);
}
