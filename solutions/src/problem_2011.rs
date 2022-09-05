impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res: i32 = 0;

        // go thru each op
        for operation in operations.iter() {
            match operation.contains("+") {
                true => res = res + 1,
                false => res = res - 1,
            }
        }

        res
    }
}
