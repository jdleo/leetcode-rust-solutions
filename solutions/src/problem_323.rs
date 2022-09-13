struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut result = 0i32;
        // graph for edges and set for visited
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();

        // build graph
        for edge in edges.into_iter() {
            (*graph.entry(edge[0]).or_insert(Vec::new())).push(edge[1]);
            (*graph.entry(edge[1]).or_insert(Vec::new())).push(edge[0]);
        }

        // go through all possible nodes
        for node in 0..n {
            // check if NOT visited yet
            if !visited.contains(&node) {
                // this will be one connected graph, and visit all nodes in it
                result += 1;
                Self::visit(node, &graph, &mut visited);
            }
        }

        result
    }

    // helper method to visit all nodes in a graph
    fn visit(current: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) {
        // make sure not already visited
        if !visited.contains(&current) {
            // mark as visited, and visit all neighbors
            visited.insert(current);
            for neighbor in graph.get(&current).unwrap_or(&Vec::new()) {
                Self::visit(*neighbor, graph, visited);
            }
        }
    }
}
