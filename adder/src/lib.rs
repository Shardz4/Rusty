pub fn add_two(a: i32) -> i32 {
    a+3
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
      let result = add(2, 2);
      assert_ne !(result, 4);
  } 


}
