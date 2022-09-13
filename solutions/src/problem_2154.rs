struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut original = original;

        // put all nums in set and keep doubling until not in seen
        let seen: HashSet<i32> = nums.into_iter().collect();
        while seen.contains(&original) {
            original <<= 1;
        }

        original
    }
}
