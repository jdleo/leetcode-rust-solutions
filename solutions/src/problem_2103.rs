struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        // to map rods to colors
        let mut rods: HashMap<u8, HashSet<char>> = HashMap::new();

        // go through rings in chunks of 2
        for chunk in rings.chars().collect::<Vec<char>>().chunks(2) {
            let color = chunk[0];
            let rod = (chunk[1] as u8) - ('0' as u8);

            // insert this color into this rod (or new hashset if not exist yet)
            (*rods.entry(rod).or_insert(HashSet::new())).insert(color);
        }

        // count only hashsets that have all 3 colors
        rods.values().filter(|set| set.len() == 3).count() as i32
    }
}
