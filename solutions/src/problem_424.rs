struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // hash map to store character frequencies
        let mut counts: HashMap<char, i32> = HashMap::new();

        // max substring length
        let mut result = 0i32;

        // go through chars
        for (index, chr) in s.chars().enumerate() {
            // +1 frequency
            *counts.entry(chr).or_insert(0) += 1;

            // check if we can continue growing the substring, meaning
            // with or without replacements, still room for k
            if result + 1 <= counts.values().max().unwrap() + k {
                // can grow the window
                result += 1;
            } else {
                // otherwise, we know we have to shrink substring
                let leftmost = index - result as usize;
                *counts.entry(s.as_bytes()[leftmost] as char).or_insert(0) -= 1;
            }
        }

        result
    }
}
