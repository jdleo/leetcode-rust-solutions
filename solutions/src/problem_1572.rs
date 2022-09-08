struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let (mut sum, n) = (0, mat.len());

        for i in 0..n {
            // add from top left, and top right
            sum += mat[i][i] + mat[i][n - i - 1];
        }

        // if n was odd, we double added the middle one
        if n & 1 == 1 {
            sum -= mat[n / 2][n / 2]
        }

        sum
    }
}
