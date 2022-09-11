struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        // current parantheses balance and total adds
        let (mut balance, mut result) = (0i32, 0i32);

        for chr in s.chars() {
            if chr == '(' {
                balance += 1;
            } else {
                // check if dont have accompanying opening
                if balance == 0 {
                    // needed to add one here
                    result += 1;
                } else {
                    balance -= 1;
                }
            }
        }

        // adds + if any are unaccounted for
        result + balance
    }
}
