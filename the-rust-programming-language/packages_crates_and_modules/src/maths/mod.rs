pub fn add(a: u32, b: u32) -> u32 {
  return a + b;
}

#[test]
fn should_add() {
  assert_eq!(add(2, 4), 6)
}
