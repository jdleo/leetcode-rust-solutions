struct Solution;
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut res = String::from("");

        for chr in address.chars() {
            match chr == '.' {
                true => res.push_str("[.]"),
                false => res.push(chr),
            }
        }

        res
    }
}
