extern crate web_server;

fn main() {
    web_server::new()
        .get("/user/:id", |request, _| {
            format!("{:?}", request.params.get("id")).into()
        })
        .launch(8080);
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_server() {
        web_server::new()
            .get("/stats/:num", |_, _| "ok".into())
            .post("/stats/:num", |_, _| "ok".into())
            .any("/stats/:num", |_, _| "ok".into());
    }
}
