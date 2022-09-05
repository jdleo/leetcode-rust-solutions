struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        // go through nums twice
        for i in 0..nums.len() * 2 {
            res.push(nums[(i % nums.len()) as usize]);
        }

        res
    }
}
