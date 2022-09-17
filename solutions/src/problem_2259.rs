struct Solution;

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        (0..number.len())
            .filter(|i| number.as_bytes()[*i] == digit as u8)
            .map(|i| (&number[..i]).to_string() + &number[i + 1..].to_string())
            .into_iter()
            .max()
            .unwrap()
    }
}
