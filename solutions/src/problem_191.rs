struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut n, mut result) = (n, 0);

        // go until n is 0
        while n > 0 {
            // count this bit (if set)
            result += n & 1;
            // shift to get next bit
            n >>= 1;
        }

        result as i32
    }
}
