impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; queries.len()];

        // go thru each query
        for (i, query) in queries.iter().enumerate() {
            // count for this query
            let mut count = 0;

            // go thru each point
            for point in points.iter() {
                // check if lies within this circle
                let (x_1, y_1, radius) = (query[0] as f64, query[1] as f64, query[2] as f64);
                let (x_2, y_2) = (point[0] as f64, point[1] as f64);
                let diff = ((x_1 - x_2).powi(2) + (y_1 - y_2).powi(2));
                if diff <= radius.powi(2) {
                    count += 1;
                }
            }
        }

        res
    }
}
