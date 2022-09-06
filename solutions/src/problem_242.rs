struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        // vector for character counts
        let mut counts: Vec<u16> = vec![0; 26];

        // count characters
        for (a, b) in s.chars().zip(t.chars()) {
            // maintain balance
            counts[a as usize - 97] += 1;
            counts[b as usize - 97] -= 1;
        }

        // all counts should be 0 (balanced)
        counts.into_iter().all(|c| c == 0)
    }
}
