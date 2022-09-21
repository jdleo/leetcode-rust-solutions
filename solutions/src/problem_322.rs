struct Solution;

use std::cmp::min;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // dp array for minimum coins for every amount from 0 to amount
        let mut dp: Vec<i32> = vec![i32::MAX; (amount + 1) as usize];

        // start at 0 because it takes 0 coins to make 0 change
        dp[0] = 0;

        // go through all possible amounts
        for current_amount in 1..=amount {
            // go through all possible coins we can use
            for i in 0..coins.len() {
                let coin = coins[i];

                // check if we can make change here
                if coin <= current_amount {
                    // take this coin only if its less coins than the change
                    // and verified via the result stored in dp
                    if dp[(current_amount - coin) as usize] != i32::MAX {
                        dp[current_amount as usize] = min(
                            dp[current_amount as usize],
                            1 + dp[(current_amount - coin) as usize],
                        );
                    }
                }
            }
        }

        // last, because we built up to "amount"s solution
        // unless its max, means we cant make change
        if dp[amount as usize] != i32::MAX {
            dp[amount as usize]
        } else {
            -1
        }
    }
}
