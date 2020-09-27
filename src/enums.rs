#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub enum State {
    On,
    Off,
    Moving,
    Stopped,
}
