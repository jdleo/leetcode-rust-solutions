struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        // collect allowed characters to set
        let allowed_set: HashSet<char> = allowed.chars().collect();

        // sum words that only have allowed characters
        words
            .iter()
            .filter(|word| word.chars().all(|c| allowed_set.contains(&c)))
            .count() as i32
    }
}
