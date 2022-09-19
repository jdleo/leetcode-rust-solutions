struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn valid_subarrays(nums: Vec<i32>) -> i32 {
        // double ended queue to keep track of tips of subarrays
        let mut deque: VecDeque<i32> = VecDeque::new();
        let mut result = 0i32;

        for num in nums.into_iter() {
            // keep poppingback as long as it's > than this number,
            // because we cannot form a continguous subarray with it
            while !deque.is_empty() && deque.back().unwrap() > &num {
                deque.pop_back();
            }

            // push this number and increment count because we can make subarrays
            // out of this entire capacity
            deque.push_back(num);
            result += deque.len() as i32;
        }

        result
    }
}
