struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        // normalize the pattern
        let normalized_pattern = Self::normalize(&pattern);

        // simply filter the words that are equal when normalized
        words
            .into_iter()
            .filter(|word| Self::normalize(word) == normalized_pattern)
            .collect()
    }

    // helper method to "normalize" each word into first index positions
    fn normalize(word: &String) -> String {
        let mut result = String::new();
        // hashmap to keep track of first character positions
        let mut positions: HashMap<char, u8> = HashMap::new();
        let mut ptr = '0' as u8;

        // go through word
        for chr in word.chars() {
            // check if already in hashmap
            if positions.contains_key(&chr) {
                // add character
                result.push(*positions.get(&chr).unwrap() as char);
            } else {
                // use ptr
                positions.insert(chr, ptr);
                result.push(ptr as char);
                ptr += 1;
            }
        }

        result
    }
}
