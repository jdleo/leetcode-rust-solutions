impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        // running product and sum
        let mut product: i32 = 1;
        let mut sum: i32 = 0;

        // go until no more digits
        while n > 0 {
            // get rightmost digit, product, sum, and shift
            let rightmost: i32 = n % 10;
            product *= rightmost;
            sum += rightmost;
            n /= 10;
        }

        product - sum
    }
}
