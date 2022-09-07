struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        // vec to keep track of last position we've seen every character
        let mut last_seen: Vec<usize> = vec![0; 26];
        for (i, c) in s.chars().enumerate() {
            last_seen[c as usize - 97] = i;
        }

        let (mut left, mut right) = (0 as usize, 0 as usize);

        // walk through string
        for (i, c) in s.chars().enumerate() {
            // check furthest we can go
            right = std::cmp::max(right, last_seen[c as usize - 97]);

            // check if we've caught up (can partition here)
            if i == right {
                // push length and reset left
                result.push((right - left + 1) as i32);
                left = i + 1;
            }
        }

        result
    }
}
