struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_common_subsequence(arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let n = arrays.len();
        let mut result: Vec<i32> = Vec::new();

        // counts for occurrences
        let mut counts: HashMap<i32, usize> = HashMap::new();

        // count all numbers
        for array in arrays.into_iter() {
            for num in array.into_iter() {
                // increment
                *(counts.entry(num).or_insert(0)) += 1;

                // check if in all arrays
                if *counts.get(&num).unwrap() == n {
                    result.push(num);
                }
            }
        }

        result
    }
}
