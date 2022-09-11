struct BrowserHistory {
    forward_history: Vec<String>,
    backward_history: Vec<String>,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            forward_history: Vec::new(),
            backward_history: Vec::from([homepage]),
        }
    }

    fn visit(&mut self, url: String) {
        // clear forward history and push to backward
        self.forward_history = Vec::new();
        self.backward_history.push(url);
    }

    fn back(&mut self, steps: i32) -> String {
        let mut steps = steps;

        // go until steps is 0 and backward history just has one left
        while steps > 0 && self.backward_history.len() > 1 {
            // pop from backward, and add to forward
            steps -= 1;
            self.forward_history
                .push(self.backward_history.pop().unwrap());
        }

        // return whatever is at backward
        self.backward_history.last().unwrap().clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let mut steps = steps;

        // go until steps is 0 or forward history has no more
        while steps > 0 && self.forward_history.len() > 0 {
            // pop from backward, and add to forward
            steps -= 1;
            self.backward_history
                .push(self.forward_history.pop().unwrap());
        }

        // return whatever is at backward
        self.backward_history.last().unwrap().clone()
    }
}
