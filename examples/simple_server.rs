extern crate http_server;

// fn handler(req: http_server::Request, resp: http_server::Response) -> impl http_server::Response {

fn main() {
    http_server::new()
        .get("/stats/:num", |_, _| "ok2".into())
        .post("/stats", |_, _| "ok".into())
        .launch(8080)
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_server() {
        http_server::new()
            .get("/stats/:num", |_, _| "ok".into())
            .post("/stats", |_, _| "ok".into());
    }
}
