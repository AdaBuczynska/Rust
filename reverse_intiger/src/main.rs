pub struct Solution;
use std::convert::TryFrom;
impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut a = 0;
    let mut x = x as i64;
    while x.abs() > 0 {
      a = a * 10;
      a = a + x % 10;
      x = x / 10
    }

    i32::try_from(a).unwrap_or(0)
  }
}
fn main() {
  assert_eq!(Solution::reverse(123),321);
 
}
