struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        // distances array
        let mut actual_distance: Vec<i32> = vec![-1; 26];

        // calculate distances
        for (index, chr) in s.chars().enumerate() {
            // check if we already saw this character
            let chr_index = (chr as u8 - b'a') as usize;
            if actual_distance[chr_index] != -1 {
                // make sure the distance equals expected distance
                if (index as i32) - actual_distance[chr_index] - 1 != distance[chr_index] {
                    return false;
                }
            } else {
                // store occurrence
                actual_distance[chr_index] = index as i32;
            }
        }

        // no issues
        true
    }
}
