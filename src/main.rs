fn main() {
  let s = String::from("Hello beautiful");
  let result = first_word(&s);

  println!("{}", result);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}
