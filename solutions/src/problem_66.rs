struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        // shadow digits
        let mut digits = digits;

        // go thru digits backward
        for i in (0..digits.len()).rev() {
            // if 9, make it a 0 and continue
            // if anything else, add one and stop
            match digits[i] {
                9 => digits[i] = 0,
                _ => {
                    digits[i] += 1;
                    break;
                }
            }
        }

        // edge case, last number was a 9 too
        if *digits.first().unwrap() == 0 {
            digits.insert(0, 1);
        }

        digits
    }
}
