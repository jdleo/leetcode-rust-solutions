impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        // current balance and number of total splits
        let mut balance: i32 = 0;
        let mut result: i32 = 0;

        // go through chars
        for character in s.chars() {
            match character {
                'R' => balance += 1,
                _ => balance -= 1,
            }

            // check if balanced (can split here)
            if balance == 0 {
                result += 1;
            }
        }

        result
    }
}
