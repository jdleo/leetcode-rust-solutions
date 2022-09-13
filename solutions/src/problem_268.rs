struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // all numbers should be paired up with indices, so the missing one is whats left
        // after xoring them all because xoring pairs = 0 and 0 xor x = x
        nums.iter()
            .enumerate()
            .fold(0, |acc, (index, num)| acc ^ (index as i32) ^ num)
            ^ (nums.len() as i32)
    }
}
