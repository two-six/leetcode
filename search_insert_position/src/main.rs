fn main() {
    let v1 = vec![1, 3, 5, 6];
    println!("{}", Solution::search_insert(v1.clone(), 5));
    println!("{}", Solution::search_insert(v1.clone(), 2));
    println!("{}", Solution::search_insert(v1.clone(), 7));
}

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        } else if nums.iter().last().unwrap() < &target {
            return nums.len() as i32;
        } else if let Some(p) = nums.iter().position(|&x| x == target) {
            return p as i32;
        } else if let Some(p) = nums.iter().position(|&x| x > target) {
            return p as i32;
        } else if let Some(p) = nums.iter().position(|&x| x < target) {
            return p as i32 - 1;
        }
        unreachable!();
    }
}
