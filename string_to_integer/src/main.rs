fn main() {
    let s1 = String::from("42");
    let s2 = String::from("    -42");
    let s3 = String::from("4192 with words");
    let s4 = String::from("words and 987");
    let s5 = String::from("-91283472332");
    let s6 = String::from("  0000000000012345678");
    let s7 = String::from("-5-");
    println!("{}", Solution::my_atoi(s1));
    println!("{}", Solution::my_atoi(s2));
    println!("{}", Solution::my_atoi(s3));
    println!("{}", Solution::my_atoi(s4));
    println!("{}", Solution::my_atoi(s5));
    println!("{}", Solution::my_atoi(s6));
    println!("{}", Solution::my_atoi(s7));
}

struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut number = String::new();
        for c in s.chars() {
            if c == ' ' && number.is_empty() {
                continue;
            } else if c == '0' && number.is_empty() {
                number.push(c);
            } else if c == '0' && (number == "-" || number == "+" || number == "0") {
                continue;
            } else if (c == '+' || c == '-') && number.is_empty() {
                number.push(c);
            } else if (c == '+' || c == '-') && !number.is_empty() {
                break;
            } else if number.len() > 12 {
                break;
            } else if c.is_ascii_digit() {
                number.push(c);
            } else {
                break;
            }
        }
        if let Ok(number) = number.parse::<i64>() {
            return if number > 2147483647 {
                2147483647
            } else if number < -2147483648 {
                -2147483648
            } else {
                number as i32
            };
        }
        0
    }
}
