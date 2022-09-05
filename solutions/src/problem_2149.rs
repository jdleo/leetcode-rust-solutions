struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        // result and odd/even pointers
        let mut result: Vec<i32> = vec![0; nums.len()];
        let (mut pos, mut neg): (usize, usize) = (0, 1);

        // build result
        for num in nums.into_iter() {
            // parity
            if num < 0 {
                // push and increment
                result[neg] = num;
                neg += 2;
            } else {
                // push and increment
                result[pos] = num;
                pos += 2;
            }
        }

        result
    }
}
