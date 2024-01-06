#![warn(clippy::pedantic)]

mod api_filter;

use proc_macro::TokenStream;

// See `api_filter::api_filter`.
#[proc_macro_attribute]
pub fn api_filter(attr: TokenStream, item: TokenStream) -> TokenStream {
    match api_filter::api_filter(attr, item) {
        Ok(x) | Err(x) => x,
    }
}
