struct Solution;

use std::cmp::min;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut current_min = prices[0];

        /*
        0. prices -> iterator
        1. map each price to the profit of that day
        2. take max
        3. force unwrap because we know itll have a max
        */

        prices
            .into_iter()
            .map(|price| {
                current_min = min(current_min, price);
                price - current_min
            })
            .max()
            .unwrap()
    }
}
