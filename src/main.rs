fn main() {
  let result = first_word("hello beautiful".to_string());

  println!("{}", result);
}

fn first_word(s: String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}
