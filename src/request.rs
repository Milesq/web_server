use crate::{HttpMethod, HttpVersion};
use std::collections::HashMap;
use std::convert::TryFrom;

#[allow(dead_code)]
#[derive(Debug, Clone)]
/// Represents http request
pub struct Request {
    method: HttpMethod,
    path: String,
    http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
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
    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    #[inline]
    pub fn header(&self, key: &str) -> std::option::Option<&String> {
        self.headers.get(&key.to_string())
    }
}

impl TryFrom<String> for Request {
    type Error = ();

    fn try_from(req: String) -> Result<Self, Self::Error> {
        if let Some([info, headers, body]) = split_http_request(req) {
            let info: Vec<&str> = info.split(' ').collect();
            let query = get_query(info[1]);

            if info.len() != 3 {
                return Err(());
            }

            let method = info[0].parse::<HttpMethod>();

            if method.is_err() {
                return Err(());
            }

            let http_version = http_ver(info[2]);
            if http_version.is_none() {
                return Err(());
            }

            let headers = parse_headers(headers);
            if headers.is_none() {
                return Err(());
            }

            Ok(Self {
                method: method.unwrap(),
                path: info[1].to_string(),
                http_version: http_version.unwrap(),
                headers: headers.unwrap(),
                params: HashMap::new(),
                query,
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

    Some([info.into(), headers, body])
}

fn parse_headers(headers: String) -> Option<HashMap<String, String>> {
    fn parse_header(s: String) -> Option<(String, String)> {
        let mut s = s.chars();
        let mut key = String::new();
        let mut next_char = 0 as char;

        while next_char != ':' {
            let tmp = s.next();

            tmp?;

            next_char = tmp.unwrap();
            key.push(next_char);
        }

        s.next();

        Some((key, s.collect()))
    }

    let mut map = HashMap::new();

    for line in headers.lines() {
        let data = parse_header(line.to_string());

        data.as_ref()?;

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

fn get_query(path: &str) -> HashMap<String, String> {
    let query: Vec<&str> = path.split('?').collect();

    if query.len() == 1 {
        return HashMap::new();
    }

    x_www_form_urlencoded(query[1])
}

pub fn x_www_form_urlencoded(path: &str) -> HashMap<String, String> {
    let mut ret = HashMap::new();

    for el in path.split('&') {
        let parameter = el.split('=').collect::<Vec<&str>>();
        ret.insert(parameter[0].to_string(), parameter[1].to_string());
    }

    ret
}
