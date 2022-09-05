struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; (2 * n) as usize];

        // go up to n
        for i in 0..n {
            res[(2 * i) as usize] = nums[i as usize];
            res[(2 * i + 1) as usize] = nums[(n + i) as usize];
        }

        res
    }
}
