struct Solution;

impl Solution {
    pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let (mut p1, mut p2, mut p3): (usize, usize, usize) = (0, 0, 0);

        // go until no more possible options
        while p1 < arr1.len() && p2 < arr2.len() && p3 < arr3.len() {
            let (n1, n2, n3) = (arr1[p1], arr2[p2], arr3[p3]);
            // check if all equal
            if n1 == n2 && n2 == n3 {
                result.push(n1);
                p1 += 1;
                p2 += 1;
                p3 += 1;
            } else {
                // move largest forward
                let min = n1.min(n2.min(n3));
                if n1 == min {
                    p1 += 1
                } else if n2 == min {
                    p2 += 1
                } else {
                    p3 += 1
                }
            }
        }

        result
    }
}
