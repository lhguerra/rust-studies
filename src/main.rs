fn main() {
  let s = "Hello beautiful";
  let result = first_word(&s);

  println!("{}", result);

  let s2 = String::from("Hello Beautiful");
  let result2 = first_word(&s2[..]);

  println!("{}", result2);
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}
