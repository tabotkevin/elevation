use crate::elevator::Elevator;
use crate::enums::{Direction, State};
use crate::user::User;
use crate::utils::sleep;

#[derive(Debug)]
pub struct Building {
    pub elevator: Elevator,
    pub max_floor: u32,
    pub min_floor: u32,
}

impl Building {
    pub fn call(&mut self, user: &User) {
        self.elevator.stops.push(user.current_floor);
        println!("Elevator called to floor {}", user.current_floor);
        self.drive(user);
    }

    pub fn drive(&mut self, user: &User) {
        self.elevator.state = State::Moving;
        // while self.elevator.state == State::On {
        loop {
            match self.elevator.direction {
                Direction::Up => {
                    println!("Elevator Going up");
                    sleep(100);
                    self.elevator.current_floor += 1;
                    println!("Elevetor is {:?}", self.elevator);

                    self.check_stops(user);
                    self.check_floor(user);

                    if self.reached_max() {
                        self.change_direction(Direction::Down);
                    }
                }
                Direction::Down => {
                    println!("Going Down");
                    self.elevator.current_floor -= 1;
                    println!("Elevetor is {:?}", self.elevator);

                    self.check_stops(user);

                    self.check_floor(user);

                    if self.reached_min() {
                        self.change_direction(Direction::Up);
                    }
                }
            }
        }
    }

    fn check_stops(&mut self, user: &User) {
        if self.elevator.stops.contains(&self.elevator.current_floor) {
            println!("Elevator stopping at {}", self.elevator.current_floor);
            self.elevator.state = State::Stopped;
            println!("Elevator state is {:?}", self.elevator.state);
            sleep(100);
            self.elevator.passengers.push(user.id);
            self.elevator.state = State::Moving;
            println!("Elevator state is {:?}", self.elevator.state);
        }
    }

    fn check_floor(&mut self, user: &User) {
        if self.elevator.current_floor == user.to_floor {
            println!("Elevator reached user floor {:?}", self.elevator);
            self.elevator.state = State::Stopped;
            println!("Elevator state is {:?}", self.elevator.state);
            self.elevator
                .passengers
                .retain(|&passenger| passenger != user.id);
            self.elevator
                .stops
                .retain(|&stop| stop != user.current_floor);
            self.elevator.state = State::Moving;
            println!("Elevator after dropping user {:?}", self.elevator);
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        println!("Elevator reached maximum floor {:?}", self.elevator);
        self.elevator.state = State::Stopped;
        println!("Elevator state is {:?}", self.elevator.state);
        self.elevator.move_to(direction);
        self.elevator.state = State::Moving;
        println!("Elevator state is {:?}", self.elevator.state);
    }

    fn reached_max(&mut self) -> bool {
        self.elevator.current_floor == self.max_floor
    }

    fn reached_min(&mut self) -> bool {
        self.elevator.current_floor == self.min_floor
    }
}
