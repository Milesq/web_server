extern crate http_server;

fn main() {
    http_server::new()
        .get("/foo/bar/foobar", |_, _| "First Handler!".into())
        .not_found(|_, _| "404 error!".into())
        .get("/foo/:parameter", |req, _| {
            format!("{:#?}", req.params).as_str().into()
        })
        .any("/foo/bar/foobar", |_, _| {
            "Like First handler, but any http method".into()
        })
        .launch(8080)
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_server() {
        http_server::new()
            .get("/stats/:num", |_, _| "ok".into())
            .post("/stats/:num", |_, _| "ok".into())
            .any("/stats/:num", |_, _| "ok".into());
    }
}
