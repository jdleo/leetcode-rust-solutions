struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        // shadow words
        let mut words = words;

        // sort unstably then by length to get them lined up into substring groups
        words.sort_unstable();
        words.sort_by_key(|word| word.len());

        // result and set to keep track of seen
        let mut result = String::new();
        let mut seen: HashSet<String> = HashSet::from(["".to_string()]);

        for word in words.into_iter() {
            // check if everything besides last character is in seen
            // and we know it should be because they're sorted into groups
            // and then by length
            if seen.contains(&word[..word.len() - 1]) {
                seen.insert(word.clone());
                result = if word.len() > result.len() {
                    word
                } else {
                    result
                }
            }
        }

        result
    }
}
