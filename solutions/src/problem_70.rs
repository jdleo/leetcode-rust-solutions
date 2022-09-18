struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // edge cases
        if n <= 2 {
            return n;
        }

        // dp array with 1,2 precalculated
        let mut dp: Vec<i32> = Vec::from([1, 2]);

        // go from 2 to n because we precalculate indices 0, 1
        for i in 2..(n as usize) {
            // we could have either came from 1 back, or 2 back
            dp.push(dp[i - 1] + dp[i - 2]);
        }

        // result in last
        *dp.last().unwrap()
    }
}
