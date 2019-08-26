#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
// When crate modifier will be stable
// this can be change to ```crate enum ...```
pub enum HttpVersion {
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

impl Default for HttpVersion {
    fn default() -> Self {
        Self::Ver11
    }
}
