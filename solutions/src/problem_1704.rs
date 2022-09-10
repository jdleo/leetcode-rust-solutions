struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        // set of vowels and vowel count "balance"
        let vowels: HashSet<char> = String::from("aeiouAEIOU").chars().collect();
        let mut balance = 0i16;

        for (index, chr) in s.chars().enumerate() {
            // check if vowel
            if vowels.contains(&chr) {
                // if first half, increment, if second half, decrement
                balance += if index >= s.len() / 2 { 1 } else { -1 }
            }
        }

        // string halves should be equally balanced with vowels
        balance == 0
    }
}
