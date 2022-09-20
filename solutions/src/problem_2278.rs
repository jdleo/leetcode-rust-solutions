struct Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut count: i32 = 0;

        // count characters that match
        for chr in s.chars() {
            count += (chr == letter) as i32;
        }

        // percentage, as 100th, rounded down
        (count * 100) / (s.len() as i32)
    }
}
