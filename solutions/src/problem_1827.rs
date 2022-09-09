struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        // result and shadowed nums
        let (mut result, mut nums) = (0i32, nums);

        // go through nums for 1 to n
        for index in 1..nums.len() {
            // check if NOT increasing
            if nums[index - 1] >= nums[index] {
                // increase this number up to +1 of last
                let diff = nums[index - 1] - nums[index] + 1;
                // add to both result and this number
                result += diff;
                nums[index] += diff;
            }
        }

        result
    }
}
