struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        // find first
        words
            .into_iter()
            .find(|w| Self::is_palindrome(w.as_bytes()))
            .unwrap_or(String::new())
    }

    // helper method to determine if palindrome
    pub fn is_palindrome(word: &[u8]) -> bool {
        let (mut left, mut right) = (0usize, word.len() - 1);

        // go until meet in middle
        while left < right {
            // check if not equal
            if word[left] != word[right] {
                return false;
            }
            left = left + 1;
            right = right - 1;
        }

        // no issues
        true
    }
}
