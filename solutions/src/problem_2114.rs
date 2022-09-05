struct Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        // 1. convert to iterator
        // 2. map each sentence to its word count (spaces + 1)
        // 3. get the max
        sentences
            .into_iter()
            .map(|sentence| sentence.matches(" ").count() + 1)
            .max()
            .unwrap() as i32
    }
}
