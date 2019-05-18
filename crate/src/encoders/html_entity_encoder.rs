use crate::encoders::Encoder;

pub struct HtmlEntityEncoder;

impl Encoder for HtmlEntityEncoder {

  fn push_encoded(b: u8, output: &mut Vec<u8>) {
    
    match b {
        b'&' => output.extend_from_slice(b"&amp;"),
        b'<' => output.extend_from_slice(b"&lt;"),
        b'>' => output.extend_from_slice(b"&gt;"),
        b'"' => output.extend_from_slice(b"&quot;"),
        b'\'' => output.extend_from_slice(b"&#x27;"),
        b'/' => output.extend_from_slice(b"&#x2F;"),
        _ => output.push(b)
    }
  }
}
