struct Solution;
use std::{collections::HashSet, iter::FromIterator};

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse = vec![
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];

        // collect words to set of morse words and get size
        HashSet::<String>::from_iter(
            words
                .into_iter()
                .map(|word| Self::to_morse(word, morse.clone())),
        )
        .len() as i32
    }

    // helper method to convert to morse
    fn to_morse(word: String, morse: Vec<&str>) -> String {
        let mut result = String::new();

        // build morse
        for c in word.chars() {
            result.push_str(morse[c as usize - 97]);
        }

        result
    }
}
