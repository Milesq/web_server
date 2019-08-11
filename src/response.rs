use std::{
    collections::HashMap,
    convert::{
        From,
        Into
    }
};

#[allow(dead_code)]
pub struct Response {
    response_code: u16,
    response_code_name: String,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
    body: String
}

impl From<&str> for Response {
    fn from(resp: &str) -> Self {
        Response {
            response_code: 200,
            response_code_name: "OK".into(),
            http_version: HttpVersion::Ver11,
            headers: HashMap::new(),
            body: resp.to_string()
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        self.body
    }
}

#[allow(dead_code)]
enum HttpVersion {
    Ver1,
    Ver11,
    Ver2
}

use HttpVersion::*;

impl Into<String> for HttpVersion {
    fn into(self) -> String {
        match self {
            Ver1 => "1.0",
            Ver11 => "1.1",
            Ver2 => "2.0",
        }.into()
    }
}
