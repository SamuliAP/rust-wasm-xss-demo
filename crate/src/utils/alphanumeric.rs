pub fn is_ascii_alphanumeric(c: char) -> bool {
    
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')
}
