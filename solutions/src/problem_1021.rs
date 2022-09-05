struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let (mut result, mut balance) = (String::new(), 0);

        for c in s.chars() {
            // 4 scenarios:
            // 0: opening, and first opening - skip
            // 1: opening, and not first - add
            // 2: closing, and last closing - skip
            // 3: closing, and not last - add
            match (c, balance) {
                ('(', 0) => balance += 1,
                ('(', _) => {
                    balance += 1;
                    result.push(c);
                }
                (')', 1) => balance -= 1,
                _ => {
                    balance -= 1;
                    result.push(c);
                }
            }
        }

        result
    }
}
