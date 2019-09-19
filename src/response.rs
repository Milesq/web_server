use std::{
    collections::HashMap,
    convert::{From, Into},
    fmt::{self, Display},
};

use crate::{HttpCode, HttpVersion};

#[allow(dead_code)]
#[derive(Debug)]
/// Represents http request
pub struct Response {
    pub response_code: HttpCode,
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
            response_code: HttpCode::_200,
            http_version: HttpVersion::Ver11,
            headers,
            body: resp.to_string(),
        }
    }
}

impl From<String> for Response {
    fn from(resp: String) -> Self {
        resp.as_str().into()
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
            get_code(self.response_code),
            self.response_code
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

fn get_code(code: HttpCode) -> i16 {
    let code = format!("{:?}", code);
    let code: Vec<&str> = code.as_str().split('_').collect();

    code[1].parse().unwrap()
}
