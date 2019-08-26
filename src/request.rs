use crate::{HttpMethod, HttpVersion};
use std::collections::HashMap;
use std::convert::TryFrom;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    method: HttpMethod,
    path: String,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
    body: String,
}

// Access to fields
impl Request {
    #[inline]
    pub fn get_method(&self) -> HttpMethod {
        self.method
    }

    #[inline]
    pub fn get_path(&self) -> String {
        self.path.clone()
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

impl TryFrom<String> for Request {
    type Error = ();

    fn try_from(req: String) -> Result<Self, Self::Error> {
        if let Some([info, headers, body]) = split_http_request(req) {
            let info: Vec<&str> = info.split(' ').collect();

            if info.len() != 3 {
                return Err(());
            }

            let method = info[0].parse::<HttpMethod>();

            if let Err(_) = method {
                return Err(());
            }

            let http_version = http_ver(info[2]);
            if let None = http_version {
                return Err(());
            }

            let headers = parse_headers(headers);
            if let None = headers {
                return Err(());
            }

            Ok(Self {
                method: method.unwrap(),
                path: info[1].to_string(),
                http_version: http_version.unwrap(),
                headers: headers.unwrap(),
                body,
            })
        } else {
            Err(())
        }
    }
}

fn split_http_request(req: String) -> Option<[String; 3]> {
    let mut lines = req.lines();
    let info = lines.next()?;
    let mut headers = String::new();

    while let Some(header) = lines.next() {
        if header == "" {
            break;
        }

        headers.push_str(format!("{}\r\n", header).as_str());
    }

    let body: String = lines.collect();

    Some([info.into(), headers.into(), body.into()])
}

fn parse_headers(headers: String) -> Option<HashMap<String, String>> {
    fn parse_header(s: String) -> Option<(String, String)> {
        let mut s = s.chars();
        let mut key = String::new();
        let mut next_char = 0 as char;

        while next_char != ':' {
            let tmp = s.next();

            if let None = tmp {
                return None;
            }

            next_char = tmp.unwrap();
            key.push(next_char);
        }

        s.next();

        Some((key, s.collect()))
    }

    let mut map = HashMap::new();

    for line in headers.lines() {
        let data = parse_header(line.to_string());

        if let None = data {
            return None;
        }

        let data = data.unwrap();

        map.insert(data.0, data.1);
    }

    Some(map)
}

fn http_ver(s: &str) -> Option<HttpVersion> {
    match s {
        "HTTP/1.0" => Some(HttpVersion::Ver1),
        "HTTP/1.1" => Some(HttpVersion::Ver11),
        "HTTP/2.0" => Some(HttpVersion::Ver2),
        _ => None,
    }
}
