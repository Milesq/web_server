fn main() {
    web_server::new()
        .get("/", Box::new(|_, _| "".into()))
        .launch(8080);
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_server() {
        web_server::new()
            .get("/stats/:num", Box::new(|_, _| "ok".into()))
            .post("/stats/:num", Box::new(|_, _| "ok".into()))
            .any("/stats/:num", Box::new(|_, _| "ok".into()));
    }
}
