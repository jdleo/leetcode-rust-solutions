struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        // if it's even, return it because n is divisor of n
        // if not, just multiply by 2
        match n & 1 {
            1 => n * 2,
            _ => n,
        }
    }
}
