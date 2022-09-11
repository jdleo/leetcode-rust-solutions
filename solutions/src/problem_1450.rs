struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        /*
        1. zip start times with end times
        2. filter only those that contain query time
        3. count those filtered items and cast as i32
        */
        start_time
            .into_iter()
            .zip(end_time)
            .filter(|(start, end)| start <= &query_time && &query_time <= end)
            .count() as i32
    }
}
