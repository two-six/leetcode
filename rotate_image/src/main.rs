fn main() {
    let mut m1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut m1);
    println!("{:?}", m1);
}

struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let matrix_copy = matrix.clone();
        let n = matrix.len();
        for i in 1..=n {
            for j in 1..=n {
                matrix[n - i][j - 1] = matrix_copy[n - j][n - i];
            }
        }
    }
}
