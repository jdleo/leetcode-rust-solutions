impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();

        // to keep track of maxes in each column and row
        let mut column_max: Vec<i32> = vec![0; m];
        let mut row_max: Vec<i32> = vec![0; n];

        // find max of each column and row
        for i in 0..m {
            for j in 0..n {
                row_max[i] = std::cmp::max(row_max[i], grid[i][j]);
                column_max[j] = std::cmp::max(column_max[j], grid[i][j]);
            }
        }

        // go back through grid and sum increases
        let mut result: i32 = 0;
        for i in 0..m {
            for j in 0..n {
                // can increase up to the minimum of the maxes
                result += std::cmp::min(row_max[i], column_max[j]) - grid[i][j];
            }
        }

        result
    }
}
