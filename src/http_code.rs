use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum HttpCode {
    _110,
    _111,
    _200,
    _301,
    _302,
    _310,
    _400,
    _404,
    _500,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpCode::_110 => "Connection Timed Out",
                HttpCode::_111 => "Connection refused",
                HttpCode::_200 => "OK",
                HttpCode::_301 => "Moved Permanently",
                HttpCode::_302 => "Found",
                HttpCode::_310 => "Too many redirects",
                HttpCode::_400 => "Bad Request",
                HttpCode::_404 => "Not Found",
                HttpCode::_500 => "Internal Server Error",
            }
            .to_string()
        )
    }
}
