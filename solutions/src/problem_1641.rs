struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        // create 2d dp array representing the result
        // at each length n (row) for each vowel (col)
        let mut dp: Vec<Vec<i32>> = Vec::new();
        for i in 0..n {
            // 1 2 3 4 5 for each row because its represented as uoiea
            // and at u, there is 1 possible
            let mut row: Vec<i32> = Vec::new();
            for j in 1..6 {
                row.push(j);
            }
            dp.push(row);
        }

        // fill rest of cells
        for i in 1..n as usize {
            for j in 1..5 {
                // take result of left and top, because to the left
                // is the characters of this length and top is all characters
                // of n-1 length
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        // result is in bottom right cell
        *dp.last().unwrap().last().unwrap()
    }
}
