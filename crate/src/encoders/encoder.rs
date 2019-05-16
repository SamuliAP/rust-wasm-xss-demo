pub trait Encoder {
  fn encode(input: &str) -> String;
  fn push_encoded(c: u8, output: &mut Vec<u8>);
}
