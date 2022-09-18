struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // most water we can find, and left and right pointers
        let (mut result, mut left, mut right) = (0i32, 0usize, height.len() - 1);

        // go until we meet in middle
        while left < right {
            // check the smaller edge, because water can only be filled up to the min
            if height[left] < height[right] {
                // set new max area
                result = std::cmp::max(result, height[left] * (right - left) as i32);
                left += 1;
            } else {
                // set new max area
                result = std::cmp::max(result, height[right] * (right - left) as i32);
                right -= 1;
            }
        }

        result
    }
}
