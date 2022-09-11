struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut left, mut right) = (0usize, s.len() - 1);

        // go until meet in the middle
        while left < right {
            // swap
            s.swap(left, right);

            // pinch to middle
            left += 1;
            right -= 1;
        }
    }
}
