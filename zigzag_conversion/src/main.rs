fn main() {
    let s1 = String::from("PAYPALISHIRING");
    let s2 = String::from("A");
    let s3 = String::from("test");
    println!("{}", Solution::convert(s1.clone(), 3));
    println!("{}", Solution::convert(s1.clone(), 4));
    println!("{}", Solution::convert(s2.clone(), 1));
    println!("{}", Solution::convert(s3.clone(), 2));
}

struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut result = String::new();
        if num_rows == 1 {
            return s;
        }
        for i in 0..num_rows as usize {
            let mut up = i == num_rows as usize - 1;
            let mut previous = i;
            if let Some(c) = s.chars().nth(i) {
                result.push(c);
            } else {
                return result;
            }
            while previous < s.len() {
                let j = if up {
                    2 * i
                } else {
                    (num_rows as usize - 1 - i) * 2
                };
                if let Some(c) = s.chars().nth(previous + j) {
                    result.push(c);
                }
                previous += j;
                if i != 0 && i != num_rows as usize - 1 {
                    up = !up;
                }
            }
        }
        result
    }
}
