struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut prices: Vec<i32> = prices;
        let mut stack: VecDeque<usize> = VecDeque::new();

        // walk thru prices
        for i in 0..prices.len() {
            // see if this current price can be applied as a discount to any we seen
            while !stack.is_empty() && prices[i] <= prices[*stack.back().unwrap()] {
                // use this one and pop it
                prices[stack.pop_back().unwrap()] -= prices[i];
            }

            // add this to stack to have discount applied to it later
            stack.push_back(i);
        }

        prices
    }
}
