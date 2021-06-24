
use crate::room::*;

pub struct Map {

    pub room: Room, // TEMP
}

impl Map {
    pub fn new() -> Map {
        Map {
            room: Room::new_test_room()
        }
    }

}
