use crate::{HttpCode, RequestHandler, Response};

pub fn redirect(path: &'static str) -> RequestHandler {
    Box::new(move |_, mut res: Response| {
        res.set_http_code(HttpCode::_308);
        res.set_header("Location", path);

        res
    })
}
