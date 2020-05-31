fn main() {
    web_server::new()
        .get(
            "/user/:id",
            Box::new(|req, _| format!("{:#?}", req.query).into()),
        )
        .launch(8080);
}
