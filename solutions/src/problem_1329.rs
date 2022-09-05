struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat.first().unwrap().len();
        let mut result = mat.clone();

        // map to store sorted diagonals
        let mut diagonals: HashMap<usize, BinaryHeap<Reverse<i32>>> = HashMap::new();

        // put all cells into diagonal heaps
        for i in 0..m {
            for j in 0..n {
                // get or default to new heap and insert
                let heap = diagonals.entry(i - j).or_insert(BinaryHeap::new());
                heap.push(Reverse(mat[i][j]));
            }
        }

        // go back and get sorted
        for i in 0..m {
            for j in 0..n {
                // fill res
                let heap = diagonals.entry(i - j).or_default();
                result[i][j] = heap.pop().unwrap().0;
            }
        }

        result
    }
}
