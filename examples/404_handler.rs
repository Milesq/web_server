use web_server::route;

fn main() {
    web_server::new()
        .get("/foo/bar/foobar", &route!(|_, _| "First Handler!".into()))
        .not_found(&route!(|_, _| "404 error!".into()))
        .get(
            "/foo/:parameter",
            &route!(|req, _| { format!("{:#?}", req.params).as_str().into() }),
        )
        .any(
            "/foo/bar/foobar",
            &route!(|_, _| { "Like First handler but any http method".into() }),
        )
        .launch(8080);
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_server() {
        use web_server::route;

        web_server::new()
            .get("/stats/:num", &route!(|_, _| "ok".into()))
            .post("/stats/:num", &route!(|_, _| "ok".into()))
            .any("/stats/:num", &route!(|_, _| "ok".into()))
            .not_found(&route!(|_, _| "Not Found!!!".into()));
    }
}
