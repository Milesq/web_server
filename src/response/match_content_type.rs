use include_url::include_raw_url;

include_raw_url!("https://raw.githubusercontent.com/abonander/mime_guess/8c7794d128c421d373db984c6fd22b91d129aed1/src/mime_types.rs");

pub fn to_ext(ext: &str) -> &'_ str {
    MIME_TYPES
        .iter()
        .find(|el| el.0 == ext)
        .map(|el| el.1[0])
        .unwrap_or("text/plain")
}
