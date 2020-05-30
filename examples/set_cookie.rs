use web_server::{route, Request, Response};

fn main() {
    web_server::new()
        .get(
            "/",
            &route!(|req: Request, mut res: Response| {
                println!("{:#?}", req.cookie("token"));
                println!("{:#?}", req.cookies);
                res.set_header("Set-Cookie", "name=hello");

                res
            }),
        )
        .launch(8080);
}
