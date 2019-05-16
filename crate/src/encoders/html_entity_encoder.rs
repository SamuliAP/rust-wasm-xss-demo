use crate::encoders::Encoder;

pub struct HtmlEntityEncoder {}

impl Encoder for HtmlEntityEncoder {

  fn encode(input: &str) -> String {

    let input_length = input.len();
    let mut output: Vec<u8> = Vec::with_capacity(input_length);
    for c in input.bytes() {
      Self::push_encoded(c, &mut output);
    }
    String::from_utf8(output).unwrap()
  }

  fn push_encoded(c: u8, output: &mut Vec<u8>) {
    
    match c {
        b'&' => output.extend_from_slice(b"&amp;"),
        b'<' => output.extend_from_slice(b"&lt;"),
        b'>' => output.extend_from_slice(b"&gt;"),
        b'"' => output.extend_from_slice(b"&quot;"),
        b'\'' => output.extend_from_slice(b"&#x27;"),
        b'/' => output.extend_from_slice(b"&#x2F;"),
        _ => output.push(c)
    }
  }
}