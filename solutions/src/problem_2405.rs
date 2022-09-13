struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        // to count partitions and also to store char history
        // starts off at 1 because at minimum has 1 partition
        let mut result = 1i32;
        let mut seen: HashSet<char> = HashSet::new();

        for chr in s.chars() {
            // check if we've seen this before
            if seen.contains(&chr) {
                // clear seen (start new partition) and count
                seen.clear();
                result += 1;
            }

            // no matter what, add this to seen
            seen.insert(chr);
        }

        result
    }
}
