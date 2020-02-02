use proc_macro_hack::proc_macro_hack;

/// Encodes given string literal to [base64] and, then, using [percent encoding] during the compile-time.
/// So, resulting static `str` can be used in [data URLs].
///
/// [base64]: https://developer.mozilla.org/en-US/docs/Web/API/WindowBase64/Base64_encoding_and_decoding
/// [percent encoding]: https://developer.mozilla.org/en-US/docs/Glossary/percent-encoding
/// [data URLs]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs
#[proc_macro_hack]
pub use data_url_encode_macro_impl::data_url_encode;
