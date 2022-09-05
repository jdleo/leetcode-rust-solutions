struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        // start backtracking, which will mutate result
        Self::backtrack(&candidates, target, 0, &mut Vec::new(), &mut result);
        result
    }

    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        // check if reached target, or still can take numbers, otherwise do nothing
        if target == 0 {
            // push copy of path
            (*result).push(path.clone());
        } else if target > 0 {
            // backtrack and take any number we can
            for index in start..candidates.len() {
                // backtrack
                (*path).push(candidates[index]);
                Self::backtrack(candidates, target - candidates[index], index, path, result);
                (*path).pop();
            }
        }
    }
}
