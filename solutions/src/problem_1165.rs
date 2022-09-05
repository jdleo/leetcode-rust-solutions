use std::collections::HashMap;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut result: i32 = 0;
        let mut last: i32 = 0;

        // index the key positions
        let mut positions: HashMap<char, usize> = HashMap::new();
        for (index, character) in keyboard.chars().enumerate() {
            positions.insert(character, index);
        }

        // go through word
        for character in word.chars() {
            // add distance travelled
            let current_position = *positions.get(&character).unwrap() as i32;
            let difference = current_position - last;
            result += difference.abs();

            // set last
            last = current_position;
        }

        result
    }
}
