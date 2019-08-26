use crate::{HttpCode, HttpVersion};
use std::collections::HashMap;
use std::convert::From;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    response_code: HttpCode,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
    body: String,
}

// Access to fields
impl Request {
    #[inline]
    pub fn get_response_code(&self) -> HttpCode {
        self.response_code
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

impl From<String> for Request {
    fn from(req: String) -> Self {
        Self {
            response_code: HttpCode::_200,
            http_version: HttpVersion::Ver11,
            headers: Default::default(),
            body: String::new(),
        }
    }
}
