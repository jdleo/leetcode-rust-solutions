struct Solution;

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        let mut current = num;
        let mut result = 0i32;

        // reverse once
        while current > 0 {
            result *= 10;
            result += current % 10;
            current /= 10;
        }

        // reverse again
        current = result;
        result = 0;
        while current > 0 {
            result *= 10;
            result += current % 10;
            current /= 10;
        }

        // double reverse == num
        result == num
    }
}
