pub struct Solution;
impl Solution {
   pub fn add_strings(num1: String, num2: String) -> String {
    let num1_chars: Vec<char> = num1.chars().collect();
    let num2_chars: Vec<char> = num2.chars().collect();
    let mut carry = 0;
    let mut result = String::new();
    let mut i = num1_chars.len().wrapping_sub(1);
    let mut j = num2_chars.len().wrapping_sub(1);

    while i < num1_chars.len() || j < num2_chars.len() || carry > 0 {
        let digit1 = if i < num1_chars.len() { num1_chars[i] as u8 - b'0' } else { 0 };
        let digit2 = if j < num2_chars.len() { num2_chars[j] as u8 - b'0' } else { 0 };
        let sum = digit1 + digit2 + carry;
        carry = sum / 10;
        let digit = (sum % 10) as u8 + b'0';

        result.push(digit as char);
        i = i.wrapping_sub(1);
        j = j.wrapping_sub(1);
    }

    result.chars().rev().collect()
}
}
fn main() {
    assert_eq!(Solution::add_strings("111".to_string(),"111".to_string()),"222".to_string());
}
