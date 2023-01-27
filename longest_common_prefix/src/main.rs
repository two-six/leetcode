use std::collections::HashSet;

fn main() {
    let strs = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let strs1 = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];
    let strs2 = vec![String::from("aa"), String::from("ab")];
    println!("{}", Solution::longest_common_prefix(strs));
    println!("{}", Solution::longest_common_prefix(strs1));
    println!("{}", Solution::longest_common_prefix(strs2));
}

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if let Some(minimum_len) = strs.iter().map(|s| s.len()).min() {
            for i in 1..=minimum_len {
                if strs
                    .iter()
                    .map(|s| &s[0..i])
                    .collect::<HashSet<&str>>()
                    .len()
                    != 1
                {
                    return strs[0][0..i - 1].to_owned();
                } else if i == minimum_len {
                    return strs[0][0..minimum_len].to_owned();
                }
            }
        }
        "".to_owned()
    }
}
