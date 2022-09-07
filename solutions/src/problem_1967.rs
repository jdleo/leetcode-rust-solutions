struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        /*
        1. turn into iterator
        2. filter each pattern which exist inside word
        3. count them
        */
        patterns
            .into_iter()
            .filter(|pattern| word.contains(pattern))
            .count() as i32
    }
}
