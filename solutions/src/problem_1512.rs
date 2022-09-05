struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        // map to count number occurrences
        let mut counts: HashMap<i32, i32> = HashMap::new();

        // go through nums
        for num in nums.iter() {
            // add identical count of numbers to result (or default to 0)
            let current_count = counts.get(num).unwrap_or(&0);
            result += current_count;
            counts.insert(*num, current_count + 1);
        }

        result
    }
}
