struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut result: i32 = 0;

        // go through all pairs bruteforce
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                // check conditions provided by problem
                if nums[i] == nums[j] && (i * j) % k as usize == 0 {
                    result += 1
                }
            }
        }

        result
    }
}
