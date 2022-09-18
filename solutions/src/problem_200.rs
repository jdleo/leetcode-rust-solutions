struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // shadow grid so we dont override original
        let mut grid = grid;

        // go through grid and count and sink islands
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = 0i32;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    result += 1;
                    Self::dfs(&mut grid, i, j, m, n);
                }
            }
        }

        result
    }

    // helper method to "sink" and count islands
    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        // check if island
        if grid[i][j] == '1' {
            // sink
            grid[i][j] = '0';

            // bounds check and dfs
            if i > 0 {
                Self::dfs(grid, i - 1, j, m, n);
            }

            if i < m - 1 {
                Self::dfs(grid, i + 1, j, m, n);
            }

            if j > 0 {
                Self::dfs(grid, i, j - 1, m, n);
            }

            if j < n - 1 {
                Self::dfs(grid, i, j + 1, m, n);
            }
        }
    }
}
