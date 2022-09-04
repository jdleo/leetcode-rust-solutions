impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        // dp array with 0 at beginning
        let mut dp: Vec<i32> = Vec::from([0]);

        // go from 1 to n
        for num in 1..=n {
            // result is divided by 0 plus rightmost bit (odd)
            dp.push(dp[(num >> 1) as usize] + (num & 1));
        }

        dp
    }
}
