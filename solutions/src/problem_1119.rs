use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn remove_vowels(s: String) -> String {
        // set of vowels
        let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();

        // filter s by characters that are not vowels and collect to string
        s.chars().filter(|c| !vowels.contains(c)).collect()
    }
}
