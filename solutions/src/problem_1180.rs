struct Solution;

impl Solution {
    pub fn count_letters(s: String) -> i32 {
        let (mut result, mut streak, s) = (0i32, 1i32, s.as_bytes());

        // go through string
        for index in 1..s.len() {
            if s[index] != s[index - 1] {
                // add current streak (all substrings using math combinatorics)
                // and reset streak
                result += (streak * (streak + 1)) >> 1;
                streak = 0;
            }

            // streak continues no matter what
            streak += 1;
        }

        // add whatever streak is left
        result + ((streak * (streak + 1)) >> 1)
    }
}
