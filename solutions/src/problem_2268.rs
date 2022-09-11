struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_keypresses(s: String) -> i32 {
        // for keypress frequency
        let mut clicks: HashMap<char, i32> = HashMap::new();
        for chr in s.chars() {
            *clicks.entry(chr).or_insert(0) += 1;
        }

        // get all of the entries
        let mut entries = clicks.into_iter().collect::<Vec<(char, i32)>>();

        // sort entries by frequency desc (to minimize amount of clicks)
        entries.sort_by(|(_, a), (_, b)| b.cmp(a));

        let mut result = 0i32;

        // go through entries
        for i in 0..entries.len() {
            // keys 0-8 can be lined up in first (1 click)
            // keys 9-18 can be lined up in second (2 clicks)
            // keys 19-26 can be lined up in third (3 clicks)
            if i < 9 {
                result += entries[i].1;
            } else if i < 18 {
                result += 2 * entries[i].1;
            } else {
                result += 3 * entries[i].1;
            }
        }

        result
    }
}
