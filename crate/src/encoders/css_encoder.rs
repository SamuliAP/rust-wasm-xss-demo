use crate::encoders::Encoder;
use crate::utils::alphanumeric;

pub struct CssEncoder;

impl Encoder for CssEncoder {

  fn push_encoded(b: u8, output: &mut Vec<u8>) {
    
    if !alphanumeric::is_ascii_alphanumeric(b as char) {
      output.extend_from_slice(b"\\HH");
    } else {
      output.push(b)
    }
  }
}
