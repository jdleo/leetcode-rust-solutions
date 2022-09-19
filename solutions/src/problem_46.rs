struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // mutable result
        let mut result: Vec<Vec<i32>> = Vec::new();
        // backtrack and fill result with mutable reference
        Self::backtrack(&nums, &mut result, &mut Vec::new());
        result
    }

    fn backtrack(nums: &Vec<i32>, result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
        // check if path is a permutation
        if path.len() == nums.len() {
            // push copy of path
            (*result).push(path.clone());
        } else {
            // go thru all candidates
            for num in nums.into_iter() {
                // make sure not already picked
                if path.contains(num) {
                    continue;
                }

                // backtrack
                (*path).push(*num);
                Self::backtrack(nums, result, path);
                (*path).pop();
            }
        }
    }
}
