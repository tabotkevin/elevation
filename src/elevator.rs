
#[derive(Debug)]
struct Elevator {
    current_floor: u32
    direction: Direction,
    passengers: vec!
}

impl Elevator {
    
    pub fn start() -> Self {
        Elevator {
            current_floor: 0
            direction: Direction::up,
            passengers: vec![]
        };
    }
}