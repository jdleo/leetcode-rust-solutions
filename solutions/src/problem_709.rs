struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        // map each character to lowercase, only if uppercase, otherwise take the char
        s.chars()
            .map(|c| {
                if 'A' <= c && c <= 'Z' {
                    (c as u8 + 32) as char
                } else {
                    c
                }
            })
            .collect()
    }
}
