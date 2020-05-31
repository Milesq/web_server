fn main() {
    web_server::new()
        .get("/foo/bar/foobar", Box::new(|_, _| "First Handler!".into()))
        .not_found(Box::new(|_, _| "404 error!".into()))
        .get(
            "/foo/:parameter",
            Box::new(|req, _| format!("{:#?}", req.params).as_str().into()),
        )
        .any(
            "/foo/bar/foobar",
            Box::new(|_, _| "Like First handler but any http method".into()),
        )
        .launch(8080);
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_server() {
        web_server::new()
            .get("/stats/:num", Box::new(|_, _| "ok".into()))
            .post("/stats/:num", Box::new(|_, _| "ok".into()))
            .any("/stats/:num", Box::new(|_, _| "ok".into()))
            .not_found(Box::new(|_, _| "Not Found!!!".into()));
    }
}
