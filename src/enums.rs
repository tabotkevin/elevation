#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
pub enum State {
    On,
    Off,
    Moving,
    Stopped,
}
