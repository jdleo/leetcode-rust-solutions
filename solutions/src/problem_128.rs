struct Solution;

use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut result = 0i32;

        // first, collect all nums to set for o(1) lookups
        let seen: HashSet<i32> = nums.clone().into_iter().collect::<HashSet<i32>>();

        // go through set
        for num in nums.into_iter() {
            // check if this is first in sequence (left not found)
            if !seen.contains(&(num - 1)) {
                let mut current = num;
                let mut count = 0i32;

                // keep counting as so long as we can continue it from seen
                while seen.contains(&current) {
                    count += 1;
                    current += 1;
                }

                // set max subsequence
                result = max(result, count);
            }
        }

        result
    }
}
