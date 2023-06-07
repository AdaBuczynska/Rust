pub struct Solution;
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut result = start;

    for i in 1..n {
        let num = start + 2 * i;
        result ^= num;
    }

    result
}
        
    }
fn main() {
    assert_eq!(Solution::xor_operation(5,0),8);
}
