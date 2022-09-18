struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        // unsigned result starting at 0
        let (mut result, mut x) = (0u32, x);

        // go all potential 32 unsigned bits
        for _ in 0..32 {
            // get rightmost bit
            let rightmost = x & 1;

            // left shift result to make room for new bit
            // and right shift x to keep taking bits
            result <<= 1;
            x >>= 1;

            // add this bit to it
            result |= rightmost;
        }

        result
    }
}
