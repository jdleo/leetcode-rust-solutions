struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        // shadow and sort by start times
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|interval| interval[0]);

        // if any have collisions, can't attend all
        for index in 1..intervals.len() {
            if intervals[index][0] < intervals[index - 1][1] {
                return false;
            }
        }

        true
    }
}
