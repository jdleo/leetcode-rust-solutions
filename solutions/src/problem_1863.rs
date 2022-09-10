struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        // start backtracking
        Self::backtrack(&nums, &mut Vec::new(), 0)
    }

    // helper method to backtrack and xor all subsets
    fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, start: usize) -> i32 {
        // this is a valid subset, so xor a copy of path
        let mut result = Self::xor_vec(path.clone());

        // go through all possible candidates
        for index in start..nums.len() {
            // draw this number and recurse
            (*path).push(nums[index]);
            result += Self::backtrack(nums, path, index + 1);
            (*path).pop();
        }

        result
    }

    // helper method to xor a vector
    fn xor_vec(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, num| acc ^ num)
    }
}
