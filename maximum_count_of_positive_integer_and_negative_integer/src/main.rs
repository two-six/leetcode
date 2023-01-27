fn main() {
    let ex1 = vec![-2, -1, -1, 1, 2, 3];
    let ex2 = vec![-3, -2, -1, 0, 0, 1, 2];
    let ex3 = vec![5, 20, 66, 1314];
    println!("{}", Solution::maximum_count(ex1));
    println!("{}", Solution::maximum_count(ex2));
    println!("{}", Solution::maximum_count(ex3));
}

struct Solution {}

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        nums.iter()
            .take_while(|&&n| n < 0)
            .count()
            .max(nums.iter().rev().take_while(|&&n| n > 0).count()) as i32
    }
}
