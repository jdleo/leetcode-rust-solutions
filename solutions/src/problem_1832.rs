struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        // collect into hashset and check if all 26 characters are used
        sentence.chars().collect::<HashSet<char>>().len() == 26
    }
}
