extern crate proc_macro;
use isahc::prelude::*;
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_raw_url(item: TokenStream) -> TokenStream {
    let mut url = item.to_string().get(1..).unwrap().to_string();
    url.pop();

    isahc::get(url).unwrap().text().unwrap().parse().unwrap()
}
