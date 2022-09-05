use std::borrow::Borrow;

struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        // start dfs at node 0
        Self::backtrack(&graph, 0, &mut Vec::from([0]), &mut result);
        // dfs will mutate result
        result
    }

    // helper method for dfs
    fn backtrack(
        graph: &Vec<Vec<i32>>,
        current: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        // check if reached the end
        if current == graph.len() - 1 {
            // add copy of path
            (*result).push(path.clone());
        } else {
            // go through all children and backtrack
            for &child in &graph[current] {
                // backtrack
                (*path).push(child);
                Self::backtrack(graph, child as usize, path, result);
                (*path).pop();
            }
        }
    }
}
