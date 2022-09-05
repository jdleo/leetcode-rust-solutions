struct Solution;

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let (mut current_capacity, mut steps) = (capacity, 0 as i32);

        // go through plants
        for (index, plant) in plants.into_iter().enumerate() {
            // walked to this index
            steps += 1;

            // check if we can't fill (if we can, just keep walking)
            if plant > current_capacity {
                // walk back to river and back
                steps += (index * 2) as i32;
                // refill
                current_capacity = capacity;
            }

            // use water
            current_capacity -= plant;
        }

        steps
    }
}
