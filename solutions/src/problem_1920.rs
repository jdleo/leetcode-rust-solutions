struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        for &num in nums.iter() {
            res.push(nums[num as usize]);
        }

        res
    }
}
