use crate::elevator::Elevator;
use crate::enums::Direction;
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
        self.elevator.moves();

        while self.elevator.is_on() {
            match self.elevator.direction {
                Direction::Up => {
                    sleep(100);
                    println!(
                        "Elevator currently at {} floor and going {:?}",
                        self.elevator.current_floor, self.elevator.direction
                    );
                    self.elevator.current_floor += 1;

                    self.check_stops(user);
                    self.check_floor(user);

                    if self.reached_max() {
                        self.change_direction(Direction::Down);
                    }
                }

                Direction::Down => {
                    sleep(100);
                    println!(
                        "Elevator currently at {} floor and going {:?}",
                        self.elevator.current_floor, self.elevator.direction
                    );
                    self.elevator.current_floor -= 1;

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
            println!(
                "Elevator stopped for a passenger at floor {}",
                self.elevator.current_floor
            );
            self.elevator.stop();
            sleep(100);
            self.elevator.passengers.push(user.id);
            println!("Elevator carried carried a passenger {:?}", self.elevator);
            self.elevator.moves();
        }
    }

    fn check_floor(&mut self, user: &User) {
        if self.elevator.current_floor == user.to_floor {
            println!("Elevator reached passenger's floor {:?}", self.elevator);
            self.elevator.stop();
            self.elevator
                .passengers
                .retain(|&passenger| passenger != user.id);
            println!(
                "Elevator dispatched passenger at {} floor",
                self.elevator.current_floor
            );
            self.elevator
                .stops
                .retain(|&stop| stop != user.current_floor);
            self.elevator.moves();
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        println!("Elevator reached maximum floor {:?}", self.elevator);
        self.elevator.stop();
        self.elevator.goto(direction);
        self.elevator.moves();
    }

    fn reached_max(&mut self) -> bool {
        self.elevator.current_floor == self.max_floor
    }

    fn reached_min(&mut self) -> bool {
        self.elevator.current_floor == self.min_floor
    }
}
