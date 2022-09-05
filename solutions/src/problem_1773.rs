struct Solution;

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        // figure out rule key index (to check items)
        let mut rule_key_index: usize = 0;
        match rule_key.as_str() {
            "type" => rule_key_index = 0,
            "color" => rule_key_index = 1,
            _ => rule_key_index = 2,
        }

        // count items that match
        items
            .iter()
            .filter(|item| item[rule_key_index] == rule_value)
            .count() as i32
    }
}
