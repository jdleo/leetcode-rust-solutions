struct Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        // sort seats and students
        seats.sort_unstable_by(|a, b| a.cmp(b));
        students.sort_unstable_by(|a, b| a.cmp(b));

        // sum differences (moves)
        students
            .iter()
            .zip(seats)
            .fold(0, |acc, (student, seat)| acc + (student - seat).abs())
    }
}
