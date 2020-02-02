extern crate proc_macro;

use syn::{parse_macro_input, LitStr};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn data_url_encode(input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as LitStr).value();
    let base64_data = base64::encode(data.as_bytes());
    let encoded_data = utf8_percent_encode(&base64_data, NON_ALPHANUMERIC).to_string();
    let lit = LitStr::new(&encoded_data, Span::call_site());

    TokenStream::from(quote!(#lit))
}