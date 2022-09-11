struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        // first, count frequencies
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for num in nums.into_iter() {
            *counts.entry(num).or_insert(0) += 1;
        }

        // return the sum of numbers that have frequency 1 (unique)
        counts
            .into_iter()
            .filter(|(_, count)| *count == 1)
            .fold(0, |acc, (num, _)| acc + num)
    }
}
