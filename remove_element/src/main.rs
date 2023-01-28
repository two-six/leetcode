fn main() {
    let mut v1 = vec![3, 2, 2, 3];
    let mut v2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    println!("{} - {:?}", Solution::remove_element(&mut v1, 3), v1);
    println!("{} - {:?}", Solution::remove_element(&mut v2, 2), v2);
}

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        while let Some(n) = nums.iter().position(|x| x == &val) {
            nums.remove(n);
        }
        nums.len() as i32
    }
}
