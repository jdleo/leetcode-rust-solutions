struct Solution;

use std::cmp::Ordering::{Equal, Greater};

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let (mut num1, mut num2) = (num1, num2);
        let mut result = 0i32;

        // keep looping until one is 0
        while num1 != 0 && num2 != 0 {
            // decrement the smaller one
            match num1.cmp(&num2) {
                Equal | Greater => num1 -= num2,
                _ => num2 -= num1,
            }
            result += 1
        }

        result
    }
}
