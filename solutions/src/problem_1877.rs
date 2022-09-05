struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;

        // sort nums (so we can minimize all sums)
        nums.sort_unstable();

        // go up to half
        for index in 0..nums.len() / 2 {
            // smallest and biggest
            let sum = nums[index] + nums[nums.len() - index - 1];

            // check if new max
            if sum > result {
                result = sum;
            }
        }

        result
    }
}
