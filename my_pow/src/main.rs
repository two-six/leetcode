fn main() {
    let x1 = 2.0;
    let x2 = 2.1;
    let x3 = 2.0;
    let x4 = 0.00001;
    let x5 = -1.0;
    println!("{}", Solution::my_pow(x1, 10));
    println!("{}", Solution::my_pow(x2, 3));
    println!("{}", Solution::my_pow(x3, -2));
    println!("{}", Solution::my_pow(x4, 2147483647));
    println!("{}", Solution::my_pow(x5, -2));
}

struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 || x == 1.0 {
            1.0
        } else if x == -1.0 {
            if n % 2 == 0 {
                1.0
            } else {
                -1.0
            }
        } else if n > 0 {
            if x < 1.0 && x > -1.0 {
                let mut res = x;
                for _ in 1..n {
                    res *= x;
                    if res == 0.0 {
                        return 0.0;
                    }
                }
                res
            } else {
                let mut res = x;
                for _ in 1..n {
                    res *= x;
                }
                res
            }
        } else {
            let mut res = x;
            for _ in n..1 {
                res /= x;
                if res == 0.0 {
                    return 0.0;
                }
            }
            res
        }
    }
}
