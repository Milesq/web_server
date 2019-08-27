extern crate http_server;

fn main() {
    http_server::new()
        .get("/ok/my/wer", |_, _| "ok2".into())
        .not_found(|_, _| "404 error!".into())
        .get("/ok/:parameter", |req, _| {
            format!("{:#?}", req.params).as_str().into()
        })
        .any("/ok/my/wer", |_, _| "any".into())
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