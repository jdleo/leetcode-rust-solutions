impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max: i32 = 0;

        for &candy in candies.iter() {
            if candy > max {
                max = candy;
            }
        }

        candies
            .iter()
            .map(|candy| candy + extra_candies >= max)
            .collect()
    }
}
