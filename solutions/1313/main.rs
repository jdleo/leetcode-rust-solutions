impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        // go through nums by pairs
        for chunk in nums.chunks(2) {
            let freq = chunk[0];
            let val = chunk[1];

            // push val freq times
            for _ in 0..freq {
                result.push(val);
            }
        }

        result
    }
}
