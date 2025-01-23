fn main() {
    let muts: String = String::from("Hello WORLD");
    let s2: &str = "Hello WORLD";


    let word: usize = first_word(s2);

}

  fn first_word(s: &string) -> &str{
    let bytes: &[u8] = s.as_bytes();
    for (i: usize, &item: u8) in bytes.iter().enumerate() {

        if item == b' ' {
            return &s[0..i];
    &s[..];
    }
  }



