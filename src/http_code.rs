use std::fmt::{self, Display};

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
                HttpCode::_301 => "",
                HttpCode::_302 => "",
                HttpCode::_310 => "",
                HttpCode::_400 => "",
                HttpCode::_404 => "",
                HttpCode::_500 => "",
            }
            .to_string()
        )
    }
}
