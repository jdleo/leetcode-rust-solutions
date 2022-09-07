struct Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        // result string and shadow collect string
        let mut result: String = String::new();

        // walk through string
        for c in s.chars() {
            // check if digit char
            if '0' <= c && c <= '9' {
                // push last character shifted by current
                let mut last_char_num = result.chars().last().unwrap() as u8;
                last_char_num += (c as u8) - ('0' as u8);

                // push new shifted character to result
                result.push(last_char_num as char);
            } else {
                // push as normal
                result.push(c);
            }
        }

        result
    }
}
