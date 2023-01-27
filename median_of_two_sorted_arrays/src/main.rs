fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    println!("{:?}", Solution::find_median_sorted_arrays(nums1, nums2));
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    println!("{:?}", Solution::find_median_sorted_arrays(nums1, nums2));
    let nums1 = vec![1];
    let nums2 = vec![2];
    println!("{:?}", Solution::find_median_sorted_arrays(nums1, nums2));
    let nums1 = vec![0, 0, 0, 0, 0];
    let nums2 = vec![-1, 0, 0, 0, 0, 0, 1];
    println!("{:?}", Solution::find_median_sorted_arrays(nums1, nums2));
}

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        match nums1.len() + nums2.len() {
            1 => {
                (nums1.iter().fold(0, |acc, val| acc + val)
                    + nums2.iter().fold(0, |acc, val| acc + val)) as f64
            }
            2 => {
                (nums1.iter().fold(0, |acc, val| acc + val)
                    + nums2.iter().fold(0, |acc, val| acc + val)) as f64
                    / 2.0
            }
            _ => {
                let mut idx = (0, 0);
                let mut saved = Vec::new();
                let median_pos = (nums1.len() + nums2.len()) / 2;
                for _ in 0..=median_pos {
                    if nums1.len() > idx.0 && (nums2.len() == idx.1 || nums1[idx.0] <= nums2[idx.1])
                    {
                        saved.push(nums1[idx.0]);
                        idx.0 += 1;
                    } else {
                        saved.push(nums2[idx.1]);
                        idx.1 += 1;
                    }
                }
                if (nums1.len() + nums2.len()) % 2 == 0 {
                    (saved[saved.len() - 1] + saved[saved.len() - 2]) as f64 / 2.0
                } else {
                    saved[saved.len() - 1] as f64
                }
            }
        }
    }
}
