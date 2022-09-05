struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // for anagram groupings
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        // normalize each string into group (insert new vec if not exist)
        for str in strs.into_iter() {
            groups
                .entry(Self::normalize(&str))
                .or_insert(Vec::new())
                .push(str);
        }

        // collect values
        groups.into_iter().map(|(_anagram, group)| group).collect()
    }

    // helper method to normalize string
    fn normalize(str: &String) -> String {
        // create vector for counting each of 26 lowercase characters
        let mut counts: Vec<i32> = vec![0; 26];

        // count
        for c in str.chars() {
            counts[c as usize - 97] += 1
        }

        // return counts as string
        counts
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}
