use std::{
    collections::HashMap,
    convert::{From, Into},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Response {
    response_code: u16,
    response_code_name: String,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
    body: String,
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

#[allow(dead_code)]
#[derive(Debug)]
enum HttpVersion {
    Ver1,
    Ver11,
    Ver2,
}

use HttpVersion::*;

impl From<HttpVersion> for String {
    fn from(ver: HttpVersion) -> Self {
        match ver {
            Ver1 => "1.0",
            Ver11 => "1.1",
            Ver2 => "2.0",
        }
        .into()
    }
}
