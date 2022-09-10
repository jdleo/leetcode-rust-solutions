use std::borrow::Borrow;

struct Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut s = s;
        let mut result = String::new();
        let mut alphabet = String::from("abcdefghijklmnopqrstuvwxyz");

        // go until string is empty and accounted for
        while !s.is_empty() {
            // go through alphabet in sequential order
            for chr in alphabet.clone().chars() {
                if s.contains(chr) {
                    // smallest/biggest character and append it
                    result.push(chr);
                    s = s.replacen(chr, "", 1);
                } else {
                    // we should remove this to save runtime
                    alphabet = alphabet.replace(chr, "");
                }
            }

            // reverse alphabet because next iteration will be opposite dir
            alphabet = alphabet.chars().rev().collect();
        }

        result
    }
}
