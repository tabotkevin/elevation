#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
pub enum Motion {
    Moving,
    Stopped,
}

#[derive(Debug, PartialEq)]
pub enum Power {
    On,
    Off,
}
