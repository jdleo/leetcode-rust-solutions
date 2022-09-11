struct Solution;

impl Solution {
    pub fn sum_of_digits(nums: Vec<i32>) -> i32 {
        // first, find minimum element
        let mut min = i32::MAX;
        for num in nums.into_iter() {
            if num < min {
                min = num
            }
        }

        // get sum of digits
        let mut sum = 0i32;
        while min > 0 {
            sum += min % 10;
            min /= 10;
        }

        // 0 if sum of digits is odd
        ((sum & 1) ^ 1) as i32
    }
}
