impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        // initialize result with first value
        let mut result: Vec<i32> = Vec::from([first]);

        // go through encoded
        for num in encoded.into_iter() {
            // push encoded number xor'd with last in result
            result.push(num ^ result.last().unwrap())
        }

        result
    }
}
