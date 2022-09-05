struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut result: String = String::new();
        let mut space_count: i32 = 0;

        for c in s.chars() {
            // check if space
            if c == ' ' {
                // increment space count and check if we've already taken k words
                space_count += 1;

                if (space_count == k) {
                    break;
                }
            }

            // add character
            result.push(c);
        }

        result
    }
}
