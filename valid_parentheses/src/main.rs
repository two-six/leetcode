fn main() {
    let s1 = String::from("()");
    let s2 = String::from("(){}[]");
    let s3 = String::from("(]");
    let s4 = String::from("([)]");
    let s5 = String::from("(){}}{");
    println!("{}", Solution::is_valid(s1));
    println!("{}", Solution::is_valid(s2));
    println!("{}", Solution::is_valid(s3));
    println!("{}", Solution::is_valid(s4));
    println!("{}", Solution::is_valid(s5));
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        let s: Vec<u8> = s
            .chars()
            .map(|c| match c {
                '(' => 0,
                ')' => 1,
                '{' => 3,
                '}' => 4,
                '[' => 6,
                ']' => 7,
                _ => 9,
            })
            .collect();
        for c in s {
            stack.push(c);
            while stack.len() >= 2 {
                if stack[stack.len() - 1] == stack[stack.len() - 2] + 1 {
                    stack.pop();
                    stack.pop();
                } else {
                    break;
                }
            }
        }
        stack.len() == 0
    }
}
