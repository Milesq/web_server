use std::collections::HashMap;

use crate::HttpVersion;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    response_code: u16,
    response_code_name: String,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
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

    #[inline]
    pub fn get_body(&self) -> String {
        self.body.clone()
    }
}
