pub struct Solution;
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
      let mut count = 0;

    for num in nums {
        let mut digits = 0;
        let mut temp = num;

        while temp != 0 {
            temp /= 10;
            digits += 1;
        }

        if digits % 2 == 0 {
            count += 1;
        }
    }

    count   
    }
}
fn main() {
    assert_eq!(Solution::find_numbers(vec![12,345,2,6,7896]),2);
}
