pub fn get_html_sanitizer_charmap() -> Vec<(char, &'static str)> {
  vec![
    ('&', "&amp;"),
    ('<', "&lt;"),
    ('>', "&gt;"),
    ('"', "&quot;"),
    ('\'', "&#x27;"),
    ('/', "&#x2F;")
  ]
}