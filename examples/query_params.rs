use web_server::route;

fn main() {
    web_server::new()
        .get(
            "/user/:id",
            &route!(|req, _| format!("{:#?}", req.query).into()),
        )
        .launch(8080);
}
