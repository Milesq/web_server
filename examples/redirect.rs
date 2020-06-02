use web_server::utils::redirect;

fn main() {
    web_server::new()
        .get("/", Box::new(|_, _| "Index Page".into()))
        .not_found(redirect("/"))
        .run();
}
