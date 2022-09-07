struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        /*
        1. split by spaces
        2. map each word to its reverse
        3. collect into string vector
        4. join back by space
        */
        s.split(' ')
            .map(|word| word.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
