fn main() {
    println!("{}", Solution::is_palindrome(121));
    println!("{}", Solution::is_palindrome(-121));
    println!("{}", Solution::is_palindrome(10));
    println!("{}", Solution::is_palindrome(9999));
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        for i in 0..s.len() / 2 {
            if s.chars().nth_back(i).unwrap() != s.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }
}
