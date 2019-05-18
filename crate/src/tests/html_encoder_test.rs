use crate::encoders::Encoder;
use crate::encoders::HtmlEntityEncoder;

#[test]
fn test_encoding_valid_ascii() {
  assert_eq!(
    HtmlEntityEncoder::encode("!#$%()*+,-.0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"), 
    "!#$%()*+,-.0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
  );
}

#[test]
fn test_encoding_encoded_ascii() {
 
  assert_eq!(
    HtmlEntityEncoder::encode("&<>\"'/"),
    "&amp;&lt;&gt;&quot;&#x27;&#x2F;"
  );
}
 