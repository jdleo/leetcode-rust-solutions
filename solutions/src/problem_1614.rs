struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth: i32 = 0;
        let mut result: i32 = 0;

        for c in s.chars() {
            // check if parentheses
            if c == '(' {
                // deeper, and set new max
                depth += 1;
                result = max(result, depth);
            } else if c == ')' {
                depth -= 1;
            }
        }

        result
    }
}
