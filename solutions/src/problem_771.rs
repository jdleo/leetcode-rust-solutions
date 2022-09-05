struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut res: i32 = 0;

        // add all jewels to hash set
        let mut jewels_set: HashSet<char> = HashSet::new();
        for j in jewels.chars() {
            jewels_set.insert(j);
        }

        // go through each stone
        for s in stones.chars() {
            // increment res if its a jewel
            res += jewels_set.contains(&s) as i32;
        }

        res
    }
}
