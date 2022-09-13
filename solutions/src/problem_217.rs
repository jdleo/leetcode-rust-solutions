struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // to store history
        let mut seen: HashSet<i32> = HashSet::new();

        // if at any point we already seen, return true
        for num in nums.iter() {
            if seen.contains(num) {
                return true;
            }

            seen.insert(*num);
        }

        false
    }
}
