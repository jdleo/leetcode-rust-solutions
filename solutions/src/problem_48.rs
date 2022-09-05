struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix.first().unwrap().len();

        // first, swap all the rows
        matrix.reverse();

        // next, swap all adjacent
        for i in 0..m {
            for j in i + 1..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
}
