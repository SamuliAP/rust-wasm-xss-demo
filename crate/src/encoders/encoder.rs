pub trait Encoder {
  
  fn push_encoded(c: u8, output: &mut Vec<u8>);
  
  fn encode(input: &str) -> String {
    let input_length = input.len();
    let mut output: Vec<u8> = Vec::with_capacity(input_length);
    for c in input.bytes() {
      Self::push_encoded(c, &mut output);
    }
    String::from_utf8(output).unwrap()
  }
}
