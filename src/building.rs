use std::{thread, time};

use crate::elevator::Elevator;
use crate::enums::{Direction, State};
use crate::user::User;

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
                        self.elevator
                            .passengers
                            .retain(|&passenger| passenger != user.id);
                        self.elevator
                            .stops
                            .retain(|&stop| stop != user.current_floor);
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
