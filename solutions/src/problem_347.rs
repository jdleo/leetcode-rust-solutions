struct Solution;

use std::{collections::HashMap, iter::FromIterator};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // count frequencies first
        let mut counts: HashMap<i32, u16> = HashMap::new();
        for num in nums.into_iter() {
            (*counts.entry(num).or_insert(0)) += 1;
        }

        // sort keys by frequency
        let mut keys: Vec<i32> = Vec::from_iter(counts.keys().cloned());
        keys.sort_unstable_by(|a, b| counts.get(b).unwrap().cmp(counts.get(a).unwrap()));

        keys.split_at(k as usize).0.to_vec()
    }
}
