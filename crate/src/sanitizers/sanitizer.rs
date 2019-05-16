pub struct Sanitizer {

  sanitizer_charmap: Vec<(char, &'static str)>
}

impl Sanitizer {

  pub fn new(sanitizer_charmap: Vec<(char, &'static str)>) -> Sanitizer {
    Sanitizer { sanitizer_charmap }
  }

  pub fn sanitize(&self, mut text: String) -> String {
    for c in &self.sanitizer_charmap {
      text = text.replace(c.0, c.1);
    }

    String::from(text)
  }
}
