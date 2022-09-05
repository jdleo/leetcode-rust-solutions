struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // first, collect all of nums2 to their index positions
        let positions: HashMap<i32, usize> = nums2
            .into_iter()
            .enumerate()
            .map(|(index, num)| (num, index))
            .collect();

        // then, map nums1 to the positions we collected from nums2
        nums1
            .iter()
            .map(|num| *positions.get(num).unwrap() as i32)
            .collect()
    }
}
