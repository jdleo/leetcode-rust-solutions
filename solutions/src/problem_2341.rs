struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::from([0, 0]);
        // count frequencies of all numbers
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for num in nums.into_iter() {
            *counts.entry(num).or_insert(0) += 1;
        }

        // go through entries
        for (_, count) in counts.into_iter() {
            // add pairs and leftovers (lonely ones) to result
            result[0] += count / 2;
            result[1] += count & 1;
        }

        result
    }
}
