struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let (mut x, mut y, mut result) = (x, y, 0);

        // go until x and y are 0 (still bits to compare)
        while x != 0 || y != 0 {
            // xor the rightmost bits, if they differ, thatll be + 1
            // then right shift them both to get the next bits
            result += ((x & 1) ^ (y & 1));
            x >>= 1;
            y >>= 1;
        }

        result
    }
}
