use crate::enums::{Direction, Motion, Power};

#[derive(Debug)]
pub struct Elevator {
    pub current_floor: u32,
    pub direction: Direction,
    pub passengers: Vec<u32>,
    pub motion: Motion,
    pub power: Power,
    pub stops: Vec<u32>,
}

impl Elevator {
    pub fn start() -> Self {
        Elevator {
            current_floor: 0,
            direction: Direction::Up,
            passengers: vec![],
            power: Power::On,
            motion: Motion::Stopped,
            stops: vec![],
        }
    }

    pub fn off(&mut self) {
        self.power = Power::Off;
    }

    pub fn on(&mut self) {
        self.power = Power::On;
    }

    pub fn stop(&mut self) {
        self.motion = Motion::Stopped;
    }

    pub fn moves(&mut self) {
        self.motion = Motion::Moving;
    }

    pub fn goto(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn is_on(&mut self) -> bool {
        self.power == Power::On
    }

    pub fn is_of(&mut self) -> bool {
        self.power == Power::Off
    }
}
