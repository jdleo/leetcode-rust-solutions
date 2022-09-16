struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut swap_index = 0usize;

        // go from 1 to n because first is always fine
        for index in 1..nums.len() {
            // check if we shouldnt swap (can move swap index up)
            if nums[swap_index] != nums[index] {
                swap_index += 1;
                nums[swap_index] = nums[index];
            }
        }

        swap_index as i32 + 1
    }
}
