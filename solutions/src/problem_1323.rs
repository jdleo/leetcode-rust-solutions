struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        /*
        cast to string
        replace first 6 to a 9
        parse back to i32
        force unwrap
        */
        num.to_string()
            .replacen("6", "9", 1)
            .parse::<i32>()
            .unwrap()
    }
}
