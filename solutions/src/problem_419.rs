struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        // shadow board and result
        let (mut board, mut result) = (board, 0i32);
        let (m, n) = (board.len(), board.first().unwrap().len());

        // go through grid
        for i in 0..m {
            for j in 0..n {
                // check if battleship, sink the battleship, and count it
                if board[i][j] == 'X' {
                    result += 1;
                    Self::backtrack(&mut board, i, j, m, n);
                }
            }
        }

        result
    }

    // helper method to backtrack through grid
    fn backtrack(board: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        // bounds check and check if battle ship
        if 0 <= i && i < m && 0 <= j && j < n && board[i][j] == 'X' {
            // sink
            board[i][j] = '.';

            // backtrack all directions
            Self::backtrack(board, i - 1, j, m, n);
            Self::backtrack(board, i + 1, j, m, n);
            Self::backtrack(board, i, j - 1, m, n);
            Self::backtrack(board, i, j + 1, m, n);
        }
    }
}
