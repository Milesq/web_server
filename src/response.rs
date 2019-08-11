use std::convert::{
    From,
    Into
};

pub struct Response {
    body: String
}

impl From<&str> for Response {
    fn from(resp: &str) -> Self {
        Response {
            body: resp.to_string()
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        self.body
    }
}
