use web_server::{Request, Response};

fn main() {
    web_server::new()
        .get(
            "/",
            Box::new(|req: Request, mut res: Response| {
                println!("{:#?}", req.cookie("token"));
                println!("{:#?}", req.cookies);
                res.set_header("Set-Cookie", "name=hello");

                res
            }),
        )
        .launch(8080);
}
