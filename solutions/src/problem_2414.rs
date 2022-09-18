struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        // turn s into byte array
        let s = s.as_bytes();
        // dp array representing longest at each index
        let mut dp: Vec<i32> = vec![1; s.len()];

        // go through bytes
        for i in 1..s.len() {
            // 2 options here:
            // 0: character is one character bigger than last
            // 1: character is not, just take existing longest
            if s[i] - s[i - 1] == 1 {
                // +1 longer substring
                dp[i] = dp[i - 1] + 1;
            }
        }

        // max substring (longest)
        dp.into_iter().max().unwrap()
    }
}
