struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[1].contains(&edges[0][0]) {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }
}
