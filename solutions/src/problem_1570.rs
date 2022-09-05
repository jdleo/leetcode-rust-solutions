use std::collections::HashMap;

struct SparseVector {
    nonZeroes: HashMap<usize, i32>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        // collect nums hashmap to store all non zero valued indices
        let mut nonZeroes: HashMap<usize, i32> = nums
            .into_iter()
            .enumerate()
            .filter(|(index, num)| num.clone() != 0)
            .collect();

        Self { nonZeroes }
    }

    fn dot_product(self, other: SparseVector) -> i32 {
        // sum the nonzeroes with each other, but the indices must exist in both vectors
        // so we will use default value as 0 which wont affect sum
        self.nonZeroes
            .into_iter()
            .fold(0, |acc, (key, value)| match other.nonZeroes.get(&key) {
                Some(otherValue) => acc + value * otherValue,
                _ => acc,
            })
    }
}
