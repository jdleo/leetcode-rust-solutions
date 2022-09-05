impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;

        // go thru each customer
        for customer in accounts.iter() {
            let mut sum = 0;

            // go thru each account in this customer
            for account in customer.iter() {
                sum += account;
            }

            if sum > res {
                res = sum;
            }
        }

        res
    }
}
