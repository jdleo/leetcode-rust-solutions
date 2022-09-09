struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words
            .into_iter()
            .filter(|word| word.starts_with(&pref))
            .count() as i32
    }
}
