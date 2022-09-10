struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut n: usize = n as usize;
        let mut result = String::new();

        // add single "a" if n is even
        if n & 1 == 0 {
            result.push('a');
            n -= 1;
        }

        // add n b's
        result.push_str(String::from_utf8(vec![b'b'; n]).as_ref().unwrap());
        result
    }
}
