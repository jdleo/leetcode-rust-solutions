struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut a, mut b) = (a, b);
        let mut result = String::new();

        // 'carry' bit
        let mut carry = 0u8;

        // go while there are still bits to add
        while !a.is_empty() || !b.is_empty() {
            // get bits
            let a_bit = a.pop().unwrap_or('0') as u8 - b'0';
            let b_bit = b.pop().unwrap_or('0') as u8 - b'0';

            // sum bits with carry bit
            let sum = a_bit + b_bit + carry;

            // remainder goes to current, divisor goes to carry
            carry = sum >> 1;
            result.push(((sum & 1) + b'0') as char);
        }

        // if carry still has something, need to put it
        if carry == 1 {
            result.push('1');
        }

        // result is reversed
        result.chars().rev().collect()
    }
}
