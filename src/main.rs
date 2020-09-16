use std::{thread, time};

#[derive(Debug)]
enum Direction {
    Up,
    Down
}

#[derive(Debug)]
enum State {
    On,
    Off,
    Moving,
    Stopped,
}

#[derive(Debug)]
struct Elevator {
    current_floor: u32,
    direction: Direction,
    // passengers: Vec<User>,
    passengers: Vec<u32>,
    state: State,
    stops: Vec<u32>
}

impl Elevator {
    
    pub fn start() -> Self {
        Elevator {
            current_floor: 0,
            direction: Direction::Up,
            passengers: vec![],
            state: State::On,
            stops: vec![]
        }
    }

    pub fn stop(&mut self) {
        self.state = State::Off;
    }

    pub fn moves(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    current_floor: u32,
    to_floor: u32
}

impl User {

    pub fn new(id: u32, name: String, current_floor: u32, to_floor: u32) -> Self {
        User {
            id: id,
            name: name,
            current_floor: current_floor,
            to_floor: to_floor
        }
    }

    pub fn notify(self, mut building: Building) {
        println!("{} notified elevator", self.name);
        building.call(&self);
    }
}

#[derive(Debug)]
struct Building {
    elevator: Elevator,
    max_floor: u32,
    min_floor: u32
}

impl Building {
    

    pub fn call(&mut self, user: &User) {
        self.elevator.stops.push(user.current_floor);
        println!("Elevator called to floor {}", user.current_floor);
        self.drive(user);
    }

    pub fn drive(&mut self, user: &User) {
        self.elevator.state = State::Moving;
        let millis = time::Duration::from_millis(100);
        // while self.elevator.state == State::On {
            loop {
            
                match self.elevator.direction {
                    Direction::Up => {
                        println!("Elevator Going up");
                        thread::sleep(millis);
                        self.elevator.current_floor += 1;
                        println!("Elevetor is {:?}", self.elevator);
                        if self.elevator.stops.contains(&self.elevator.current_floor) {
                            println!("Elevator stopping at {}", self.elevator.current_floor);
                            self.elevator.state = State::Stopped;
                            println!("Elevator state is {:?}", self.elevator.state);
                            thread::sleep(millis);
                            // self.elevator.passengers.push(&user);
                            self.elevator.passengers.push(user.id);
                            self.elevator.state = State::Moving;
                            println!("Elevator state is {:?}", self.elevator.state);
                            continue;
                        }

                        if self.elevator.current_floor == user.to_floor {
                            println!("Elevator reached user floor {:?}", self.elevator);
                            self.elevator.state = State::Stopped;
                            println!("Elevator state is {:?}", self.elevator.state);
                            self.elevator.passengers.retain(|&passenger| passenger != user.id);
                            self.elevator.stops.retain(|&stop| stop != user.current_floor);
                            self.elevator.state = State::Moving;
                            println!("Elevator after dropping user {:?}", self.elevator);
                            continue;
                        }

                        if self.elevator.current_floor == self.max_floor {
                            println!("Elevator reached maximum floor {:?}", self.elevator);
                            self.elevator.state = State::Stopped;
                            println!("Elevator state is {:?}", self.elevator.state);
                            self.elevator.moves(Direction::Down);
                            self.elevator.state = State::Moving;
                            println!("Elevator state is {:?}", self.elevator.state);
                            continue;
                        }
                    }
                    Direction::Down => {
                        println!("Going Down");
                        thread::sleep(millis);
                        self.elevator.current_floor -= 1;
                        println!("Elevetor is {:?}", self.elevator);
                        if self.elevator.stops.contains(&self.elevator.current_floor) {
                            println!("Elevator stopping at {}", self.elevator.current_floor);
                            thread::sleep(millis);
                            continue;
                        }
                        if self.elevator.current_floor == self.min_floor {
                            self.elevator.moves(Direction::Up);
                            continue;
                        }
                    }
                }
        }
    }
}

fn main() {

    let building = Building{
        elevator: Elevator::start(),
        max_floor: 10,
        min_floor: 0
    };
    
    let user1 : User = User::new(1, String::from("Kevin"), 2, 4);
    user1.notify(building);

    
}
