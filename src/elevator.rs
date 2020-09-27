use crate::enums::{Direction, State};

#[derive(Debug)]
pub struct Elevator {
    pub current_floor: u32,
    pub direction: Direction,
    // passengers: Vec<User>,
    pub passengers: Vec<u32>,
    pub state: State,
    pub stops: Vec<u32>,
}

impl Elevator {
    pub fn start() -> Self {
        Elevator {
            current_floor: 0,
            direction: Direction::Up,
            passengers: vec![],
            state: State::On,
            stops: vec![],
        }
    }

    pub fn stop(&mut self) {
        self.state = State::Off;
    }

    pub fn moves(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
