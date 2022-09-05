struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        // sum and expand at each index (counting for odd and even length ones)
        (0..s.len()).fold(0, |acc, index| {
            acc + Self::expand(s.as_bytes(), index as i32, index)
                + Self::expand(s.as_bytes(), index as i32, index + 1)
        })
    }

    // helper method to count palindromes anchored at i,j
    fn expand(s: &[u8], mut i: i32, mut j: usize) -> i32 {
        let mut result: i32 = 0;

        // keep expanding
        while i >= 0 && j < s.len() && s[i as usize] == s[j] {
            i -= 1;
            j += 1;
            result += 1;
        }

        result
    }
}
