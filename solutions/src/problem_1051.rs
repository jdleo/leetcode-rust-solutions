struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        // clone and sort
        let mut expected = heights.clone();
        expected.sort_unstable();

        // count all where different
        (0..heights.len())
            .into_iter()
            .fold(0, |acc, i| acc + (heights[i] != expected[i]) as i32)
    }
}
