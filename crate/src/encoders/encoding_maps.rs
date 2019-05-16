pub static HTML_ENCODING_MAP: [(char, &'static str); 6] = [
  ('&', "&amp;"),
  ('<', "&lt;"),
  ('>', "&gt;"),
  ('"', "&quot;"),
  ('\'', "&#x27;"),
  ('/', "&#x2F;"),
];
