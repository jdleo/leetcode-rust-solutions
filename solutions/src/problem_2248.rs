struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        // hashmap for counting frequencies
        let n = nums.len() as i32;
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();

        // go through all nums in each array
        for row in nums.into_iter() {
            for num in row.into_iter() {
                // increment
                *counts.entry(num).or_insert(0) += 1;
                // check if in ALL arrays
                if *counts.get(&num).unwrap() == n {
                    result.push(num);
                }
            }
        }
        result.sort_unstable();
        result
    }
}
