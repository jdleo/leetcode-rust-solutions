struct Solution;

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // buckets of scores representing student : list of top 5 scores
        let mut buckets: HashMap<i32, BinaryHeap<i32>> = HashMap::new();

        // fill buckets
        for item in items.into_iter() {
            let id = item[0];
            let score = item[1];
            (*buckets.entry(id).or_insert(BinaryHeap::new())).push(-score);

            // check if min heap is over capacity (top 5), evict smallest (top)
            if let Some(heap) = buckets.get_mut(&id) {
                if heap.len() > 5 {
                    heap.pop();
                }
            }
        }

        // average all heaps
        let mut result: Vec<Vec<i32>> = buckets
            .iter()
            .map(|(id, scores)| {
                Vec::from([
                    id,
                    &(scores.into_iter().fold(0, |acc, score| acc + -score) / scores.len() as i32),
                ])
                .into_iter()
                .copied()
                .collect()
            })
            .into_iter()
            .collect();

        // sort by id
        result.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        result
    }
}
