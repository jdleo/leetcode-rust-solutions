use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // map to store num->indices
        let mut map: HashMap<i32, i32> = HashMap::new();

        // iterate thru nums
        for (i, num) in nums.iter().enumerate() {
            // option 1: complement exists in map, return it
            // option 2: it doesn't, push num and index to map
            match map.get(&(target - *num)) {
                Some(&idx) => return vec![i as i32, idx],
                None => map.insert(*num, i as i32),
            };
        }

        vec![]
    }
}
