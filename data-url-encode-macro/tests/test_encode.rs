use data_url_encode_macro::data_url_encode;

const HTML: &str = data_url_encode!("<h1>Hello, World!</h1>");

#[test]
fn test_encode() {
    assert_eq!(HTML, "PGgxPkhlbGxvLCBXb3JsZCE8L2gxPg%3D%3D");
}
