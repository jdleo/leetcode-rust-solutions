struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        // target frequency and frequency map
        let target = nums.len() / 2;
        let mut counts: HashMap<i32, usize> = HashMap::new();

        // count frequencies
        for num in nums.into_iter() {
            // increment
            *counts.entry(num).or_insert(0) += 1;
            // check if found
            if *counts.get(&num).unwrap() == target {
                return num;
            }
        }

        0
    }
}
