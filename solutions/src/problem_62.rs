use std::borrow::Borrow;

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);

        // create 2d array filled with 1s
        let mut DP: Vec<Vec<i32>> = vec![1; m].iter().map(|_| vec![1; n]).collect();

        // go through all besides first row and col (those should be 1)
        for i in 1..m {
            for j in 1..n {
                // paths here is paths from top + paths from left
                DP[i][j] = DP[i - 1][j] + DP[i][j - 1];
            }
        }

        // last cell has final
        *DP.last().unwrap().last().unwrap()
    }
}
