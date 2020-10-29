use crate::building::Building;

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub current_floor: u32,
    pub to_floor: u32,
}

impl User {
    pub fn new(id: u32, name: String, current_floor: u32, to_floor: u32) -> Self {
        User {
            id: id,
            name: name,
            current_floor: current_floor,
            to_floor: to_floor,
        }
    }

    pub fn notify(self, mut building: Building) {
        println!("{} notified elevator", self.name);
        building.call(&self);
    }
}
