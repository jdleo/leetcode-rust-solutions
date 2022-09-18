struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        // count frequencies, only if even
        for num in nums.into_iter() {
            if num & 1 == 0 {
                *counts.entry(num).or_insert(0) += 1;
            }
        }

        // return key with max frequency,
        // if equal, return smaller element
        // if no max (no evens), return -1
        counts
            .into_iter()
            .max_by_key(|(num, count)| (*count, -*num))
            .unwrap_or((-1, -1))
            .0
    }
}
