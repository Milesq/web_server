use std::{
    collections::HashMap,
    convert::{From, Into},
    fmt::{self, Display},
    fs::File,
    io::Read,
    path::Path,
};

use crate::{HttpCode, HttpVersion};

#[derive(Debug, Clone)]
pub enum Body {
    Raw(Vec<u8>),
    S(String),
}

use Body::*;

impl Body {
    pub fn unwrap_raw(self) -> Vec<u8> {
        match self {
            Raw(v) => v,
            S(_) => panic!(""),
        }
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            S(s) => write!(f, "{}", s.as_str()),
            Raw(_) => panic!("Raw body cannot be display!!"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
/// Represents http request
pub struct Response {
    pub response_code: HttpCode,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Body,
}

#[allow(clippy::new_without_default)]
impl Response {
    pub fn new() -> Self {
        "".into()
    }

    #[inline]
    pub fn header(&self, key: &str) -> std::option::Option<&String> {
        self.headers.get(&key.to_string())
    }

    #[inline]
    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    #[inline]
    pub fn set_http_code(&mut self, resp_code: HttpCode) -> &mut Self {
        self.response_code = resp_code;
        self
    }

    #[inline]
    pub fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = S(body.to_string());
        self
    }

    #[inline]
    pub fn set_raw_body(&mut self, body: Vec<u8>) -> &mut Self {
        self.body = Raw(body);
        self
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
            body: S(resp.to_string()),
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

        if let S(body) = &self.body {
            write!(f, "{}\r\n{}\r\n\r\n{}", info, headers, body.to_string())
        } else {
            let headers_result = write!(f, "{}\r\n{}\r\n\r\n", info, headers);
            let body = self.body.clone().unwrap_raw();

            unsafe {
                f.write_str(std::str::from_utf8_unchecked(&body))
                    .and(headers_result)
            }
        }
    }
}

impl From<&Path> for Response {
    fn from(value: &Path) -> Self {
        if !value.is_file() {
            println!("File {:?} doesn't exists!", value);

            Response {
                response_code: HttpCode::_404,
                ..Response::new()
            }
        } else {
            let mut f = File::open(value.display().to_string()).unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();

            Response {
                body: Raw(buffer),
                ..Response::new()
            }
        }
    }
}

fn get_code(code: HttpCode) -> i16 {
    let code = format!("{:?}", code);
    let code: Vec<&str> = code.as_str().split('_').collect();

    code[1].parse().unwrap()
}
