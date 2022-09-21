struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // dp array representing longsest subsequence ending at each index
        let mut dp: Vec<i32> = vec![1; nums.len()];

        // go thru all possible endings
        for right in 1..nums.len() {
            for left in 0..right {
                // check if we can continue a subsequence
                if nums[left] < nums[right] {
                    // we have two choices here, continue max from left
                    // or just take already at right
                    dp[right] = std::cmp::max(dp[right], 1 + dp[left]);
                }
            }
        }

        // max subsequence
        dp.into_iter().max().unwrap()
    }
}
