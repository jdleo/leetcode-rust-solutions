struct ParkingSystem {
    spots: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        // spots are 0 (dummy) 1 (big) 2 (medium) 3 (small)
        Self {
            spots: Vec::from([0, big, medium, small]),
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        // -1 spot for this car type
        self.spots[car_type as usize] -= 1;
        // check if there's enough spots
        self.spots[car_type as usize] >= 0
    }
}
