use std::{
    collections::HashMap,
    convert::{From, Into},
    fmt::{self, Display},
};

use crate::HttpVersion;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Response {
    pub response_code: u16,
    pub response_code_name: String,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[allow(clippy::new_without_default)]
impl Response {
    pub fn new() -> Self {
        "".into()
    }

    #[inline]
    pub fn header(&self, key: String) -> std::option::Option<&String> {
        self.headers.get(&key)
    }
}

impl From<&str> for Response {
    fn from(resp: &str) -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Server".into(), "serve".into());
        headers.insert("X-Powered-By".into(), "serve".into());
        headers.insert("Connection".into(), "keep-alive".into());

        Response {
            response_code: 200,
            response_code_name: "OK".into(),
            http_version: HttpVersion::Ver11,
            headers,
            body: resp.to_string(),
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = format!(
            "HTTP/{} {} {}",
            String::from(self.http_version),
            self.response_code,
            self.response_code_name
        );

        let headers = {
            let mut headers = String::new();

            for (header, value) in self.headers.iter() {
                headers.push_str(format!("{}: {}\r\n", header, value).as_str());
            }

            headers.trim_end().to_string()
        };

        write!(f, "{}\r\n{}\r\n\r\n{}", info, headers, self.body)
    }
}
