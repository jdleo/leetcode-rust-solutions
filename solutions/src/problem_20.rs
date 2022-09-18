struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // matching tokens and stack
        let closing = HashMap::from([('}', '{'), (']', '['), (')', '(')]);
        let mut stack: VecDeque<char> = VecDeque::new();

        // go thru chars
        for chr in s.chars() {
            // check if closing
            if closing.contains_key(&chr) {
                // check if match to closing is at top of stack
                if stack.is_empty() || stack.pop_back().unwrap() != *closing.get(&chr).unwrap() {
                    return false;
                }
            } else {
                // just add to stack
                stack.push_back(chr);
            }
        }

        // lastly, make sure nothing in stack (all accounted for)
        stack.is_empty()
    }
}
