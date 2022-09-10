struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        // if odd, add a single 0 because it doesnt affect sum
        if n & 1 == 1 {
            result.push(0);
        }

        // half of n, and add balanced numbers each time (all unique)
        for index in 1..=n / 2 {
            result.push(index);
            result.push(-index);
        }

        result
    }
}
