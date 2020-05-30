use web_server::route;

fn main() {
    web_server::new()
        .get("/", &route!(|_, _| "".into()))
        .launch(8080);
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_server() {
        web_server::new()
            .get("/stats/:num", &route!(|_, _| "ok".into()))
            .post("/stats/:num", &route!(|_, _| "ok".into()))
            .any("/stats/:num", &route!(|_, _| "ok".into()));
    }
}
