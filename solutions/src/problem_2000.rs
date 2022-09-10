struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let (mut result, mut did_reverse) = (String::new(), false);

        // walk through string
        for chr in word.chars() {
            // add this character
            result.push(chr);

            // check if this is character and have not reversed
            if chr == ch && !did_reverse {
                // reverse result as is and just continue
                result = result.chars().rev().collect();
                did_reverse = true;
            }
        }

        result
    }
}
