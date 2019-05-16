use crate::sanitizers::Sanitizer;
use crate::sanitizers::sanitizer_charmaps;

pub struct SanitizerFactory;

impl SanitizerFactory {

  pub fn html_sanitizer() -> Sanitizer {
    Sanitizer::new(sanitizer_charmaps::get_html_sanitizer_charmap())
  }
}
