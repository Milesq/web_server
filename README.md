# Http Server lib for Rust

[![Build Status](https://travis-ci.com/Milesq/web_server.svg?branch=master)](https://travis-ci.com/Milesq/http_server)

## web_server is a small, dependency-less crate for creating HTTP servers.

### When you coding the backend using Rust, the most annoying thing could be the size of a freamwork and the time needed to compile the application

### The web_server package fixes these problems. web_server has no dependencies, but allows you to create full-fledged servers


#### First server using web_server

```rust
extern crate web_server;
web_server::new()
   .get("/", |request: web_server::Request, mut response: web_server::Response|
        "Hello World!".into())
   .launch(80)
   .unwrap();
```

#### It's easy!
#### First you must create instance of HttpServer

```rust
web_server::new()
```
#### then you can declare your endpoints. E.g.

```rust
.get("/your/path", |request, default_response| {
    // There place your logic
    // This function returns Response
    "response text".into()
})
.post("/your/path", |_, _| "Handler for POST method")
.route(web_server::HttpMethod::DELETE, "/your/path", |_, _| "Handler for DELETE method")
.any("/your/path", |_, _| "Handler for any method")
```

#### Now you must run server by launch method
```rust
.launch(PORT).unwrap()
```

## Receiving post data
e.g.
```rust
    use web_server::decoders::x_www_form_urlencoded;

    web_server::new()
        .post("/add-user", |req, _| {
            println!("{}", req.get_body());
            let body: HashMap<String, String> = x_www_form_urlencoded(req.get_body().as_str());
            format!(
                "Add new user: {}",
                body.get("user").unwrap_or(&String::from("Error!"))
            )
            .into()
        })
        .launch(8080)
        .unwrap();
```

Read examples/ to know more!
