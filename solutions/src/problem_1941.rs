struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        // first, count all occurrences
        let mut counts: HashMap<char, i32> = HashMap::new();
        for chr in s.chars() {
            *counts.entry(chr).or_insert(0) += 1;
        }

        // check if all occurrences are equal
        counts.values().collect::<HashSet<&i32>>().len() == 1
    }
}
