pub struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<_> = nums1.into_iter().collect();
    let set2: HashSet<_> = nums2.into_iter().collect();
    let intersection: Vec<_> = set1.intersection(&set2).cloned().collect();

    intersection
    }
}
fn main() {
    assert_eq!(Solution::intersection(vec![4,9,5],vec![9,4,9,8,4]),vec![9,4]);
}
