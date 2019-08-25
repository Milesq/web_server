use std::{
    collections::HashMap,
    convert::{From, Into},
    fmt::{
        self,
        Display
    }
};

use crate::HttpVersion;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Response {
    response_code: u16,
    response_code_name: String,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    #[inline]
    pub fn get_response_code(&self) -> u16 {
        self.response_code
    }

    #[inline]
    pub fn get_response_code_name(&self) -> String {
        self.response_code_name.clone()
    }

    #[inline]
    pub fn get_http_version(&self) -> HttpVersion {
        self.http_version
    }

    #[inline]
    pub fn get_header(&self, key: String) -> std::option::Option<&String> {
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
