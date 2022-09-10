struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        // get i and j
        let (i, j) = (coordinates.as_bytes()[0], coordinates.as_bytes()[1]);

        // flatten, and check if odd
        (i + j) & 1 == 1
    }
}
