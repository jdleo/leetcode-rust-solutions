struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1; nums.len()];
        let (n, mut left, mut right) = (nums.len(), 1, 1);

        // go through nums forward and back
        for i in 0..n {
            // multiply into result
            result[i] *= left;
            result[n - i - 1] *= right;

            // multiply into running products
            left *= nums[i];
            right *= nums[n - i - 1];
        }

        result
    }
}
