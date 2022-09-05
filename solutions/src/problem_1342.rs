struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut x = num;
        let mut res: i32 = 0;

        if x == 0 {
            return 0;
        }

        // go until num is 0 (we keep reducing it)
        while x > 0 {
            // if it's odd, that's one extra step
            res += 1 + (x & 1);
            x >>= 1;
        }

        res - 1
    }
}
