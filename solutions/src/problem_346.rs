use std::collections::VecDeque;

struct MovingAverage {
    queue: VecDeque<i32>,
    size: i32,
    sum: i32,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            queue: VecDeque::new(),
            size,
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        // push this number to queue and add sum
        self.queue.push_back(val);
        self.sum += val;

        // check if we're over max capacity
        if self.queue.len() as i32 > self.size {
            // popleft and subtract from sum
            self.sum -= self.queue.pop_front().unwrap();
        }

        // average
        self.sum as f64 / self.queue.len() as f64
    }
}
