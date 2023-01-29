fn main() {
    let x1 = 123;
    let x2 = -123;
    let x3 = 120;
    let x4 = 1534236469;
    println!("{}", Solution::reverse(x1));
    println!("{}", Solution::reverse(x2));
    println!("{}", Solution::reverse(x3));
    println!("{}", Solution::reverse(x4));
    println!("{}", Solution::reverse(0));
}

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x > -10 && x < 10 {
            return x;
        }
        let s = x.to_string();
        let res = if x.is_positive() {
            s.chars().rev().collect::<String>().parse::<i64>().unwrap()
        } else {
            -(&s[1..]
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i64>()
                .unwrap())
        };
        if res > i32::max_value() as i64 || res < i32::min_value() as i64 {
            0
        } else {
            res as i32
        }
    }
}
