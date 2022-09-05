struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut bar: u8 = 1;

        // go through each character
        for c in s.chars() {
            // check if bar first
            if c == '|' {
                // toggle
                bar ^= 1;
            } else if c == '*' {
                // only sum if bar is toggled on
                if bar == 1 {
                    result += 1
                }
            }
        }

        result
    }
}
