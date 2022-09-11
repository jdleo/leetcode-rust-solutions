struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        // x,y coordinates starting at (0,0)
        let (mut x, mut y) = (0, 0);

        // go thru moves and map UDLR to moves
        for chr in moves.chars() {
            match chr {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                _ => x += 1,
            }
        }

        // make sure back at (0,0)
        (x, y) == (0, 0)
    }
}
