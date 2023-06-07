pub struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {return x}
      let mut lower = 1;
      let mut upper =  46341;
      while upper > 1 + lower {
          let new = (lower + upper) / 2;
          if new*new > x {upper = new}
          else {lower = new}
      }
      return lower
    }
}
fn main() {
    assert_eq!(Solution::my_sqrt(4),2);
    assert_eq!(Solution::my_sqrt(8),2);
    assert_eq!(Solution::my_sqrt(0),0);
}
