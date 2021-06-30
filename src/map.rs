
use crate::room::*;
use crate::floor::*;

pub struct Map {
    //pub floors: [Floor; 1],

    pub room: Room, // TEMP
}

impl Map {
    pub fn new() -> Map {
        Map {
            //floors: [Floor::test_floor()],
            room: Room::new_test_room(),
        }
    }

    //pub fn current_room() -> Room {}

}
