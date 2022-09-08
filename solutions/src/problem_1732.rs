struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut result = 0i32;
        let mut height = 0i32;

        // climb
        for climb in gain.into_iter() {
            // set new max
            height += climb;
            result = if height > result { height } else { result }
        }

        result
    }
}
