fn main() {
    println!(
        "{}",
        Solution::length_of_last_word(String::from("Hello World"))
    );
    println!(
        "{}",
        Solution::length_of_last_word(String::from("   fly me    to   the moon"))
    );
    println!(
        "{}",
        Solution::length_of_last_word(String::from("luffy is still joyboy"))
    );
}

struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}
