struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut result: i32 = 0;
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for num in nums.iter() {
            // |num - x| = k or num - x = k, num + x = k
            // or, x = num - k, or x = num + k
            let (target1, target2) = (num - k, num + k);

            // sum pairs (or insert 0 as default)
            result += *counts.get(&target1).get_or_insert(&0);
            result += *counts.get(&target2).get_or_insert(&0);

            // increment this num
            *counts.entry(*num).or_insert(0) += 1;
        }

        result
    }
}
