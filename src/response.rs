use std::{
    collections::HashMap,
    convert::{From, Into},
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
        self.response_code_name
    }

    #[inline]
    pub fn get_http_version(&self) -> HttpVersion {
        self.http_version
    }

    #[inline]
    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers
    }

    #[inline]
    pub fn get_header(&self, key: String) -> std::option::Option<&String> {
        self.headers.get(&key)
    }
}

impl From<&str> for Response {
    fn from(resp: &str) -> Self {
        Response {
            response_code: 200,
            response_code_name: "OK".into(),
            http_version: HttpVersion::Ver11,
            headers: HashMap::new(),
            body: resp.to_string(),
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        let info = format!(
            "HTTP/{} {} {}",
            String::from(self.http_version),
            self.response_code,
            self.response_code_name
        );

        let headers = {
            let mut headers = String::new();

            for (header, value) in self.headers {
                headers.push_str(format!("{}: {}\r\n", header, value).as_str());
            }

            headers = headers.trim_end().to_string();
            headers
        };

        format!("{}\r\n{}\r\n\r\n{}", info, headers, self.body)
    }
}
