fn main() {
    let v1 = vec![2, 7, 11, 15];
    let v2 = vec![3, 2, 4];
    let v3 = vec![3, 3];
    println!("{:?}, {:?}", v1.clone(), Solution::two_sum(v1, 9));
    println!("{:?}, {:?}", v2.clone(), Solution::two_sum(v2, 6));
    println!("{:?}, {:?}", v3.clone(), Solution::two_sum(v3, 6));
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums[i + 1..].iter().enumerate() {
                if n1 + n2 == target {
                    return vec![i as i32, (i + 1 + j) as i32];
                }
            }
        }
        unreachable!()
    }
}
