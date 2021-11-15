use std::collections::HashMap;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut res: i32 = 0;

        // go through nums
        for &num in nums.iter() {
            // add count of this num to res, then increment after
            res += *map.entry(num).or_insert(0);
            *map.get_mut(num).unwrap() += 1;
        }

        res
    }
}
