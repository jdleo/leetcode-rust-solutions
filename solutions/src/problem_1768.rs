struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (mut i, mut j) = (0usize, 0usize);
        let mut result = String::new();

        // go while either still have characters to merge
        while i < word1.len() || j < word2.len() {
            // merge
            if i < word1.len() {
                result.push(word1[i] as char);
                i += 1;
            }

            if j < word2.len() {
                result.push(word2[j] as char);
                j += 1;
            }
        }

        result
    }
}
