struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        // count frequencies
        let mut counts: HashMap<i32, u16> = HashMap::new();
        for num in nums.into_iter() {
            *counts.entry(num).or_insert(0) += 1;
        }

        // all values must be even (evenly paired)
        counts.values().all(|value| value & 1 == 0)
    }
}
