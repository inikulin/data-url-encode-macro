# data-url-encode-macro

Rust macro to generate base64+percent-encoded strings suitable for data URLs in compile time.

## Example

```rust
use data_url_encode_macro::data_url_encode;

const HTML: &str = data_url_encode!("<h1>Hello, World!</h1>");
```
