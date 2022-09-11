struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        // shadow nums and create two pointers
        let mut nums = nums;
        let (mut left, mut right) = (0usize, nums.len() - 1);

        // go until meet in middle
        while left < right {
            // 4 scenarios:
            // 0: left even, right even: move left forward
            // 1: left odd, right odd: move right forward
            // 2: left even, right odd: move left, right forward
            // 3: left odd, right even: swap, move left, right forward
            match (nums[left] & 1, nums[right] & 1) {
                (0, 0) => left += 1,
                (1, 1) => right -= 1,
                (0, 1) => {
                    left += 1;
                    right -= 1;
                }
                (_, _) => {
                    nums.swap(left, right);
                    left += 1;
                    right -= 1;
                }
            }
        }

        nums
    }
}
