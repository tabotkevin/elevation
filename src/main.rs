mod building;
mod elevator;
mod enums;
mod user;

use crate::building::Building;
use crate::elevator::Elevator;
use crate::user::User;

fn main() {
    let building = Building {
        elevator: Elevator::start(),
        max_floor: 10,
        min_floor: 0,
    };

    let user1: User = User::new(1, String::from("Kevin"), 2, 4);
    user1.notify(building);
}
