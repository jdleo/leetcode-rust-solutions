struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        // convert dates to days since 01-01
        let arrive_alice = Self::days_since_beginning(arrive_alice);
        let leave_alice = Self::days_since_beginning(leave_alice);
        let arrive_bob = Self::days_since_beginning(arrive_bob);
        let leave_bob = Self::days_since_beginning(leave_bob);

        // get crossover
        let crossover = min(leave_bob, leave_alice) - max(arrive_bob, arrive_alice);

        // return 0 if crossover is negative
        return max(crossover + 1, 0);
    }

    // helper method to convert date string into days since 01-01
    pub fn days_since_beginning(date: String) -> i32 {
        // month->day mapping
        let month_days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let month = date.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let day = date.split("-").nth(1).unwrap().parse::<i32>().unwrap();

        let mut result = 0i32;

        // go through months
        for i in 1..=month {
            // check if i is month
            if i == month {
                // just add days
                result += day;
            } else {
                // add days in this month
                result += month_days[i as usize];
            }
        }

        result
    }
}
