use crate::room::*;

pub struct Floor {

    // This is an option because not every tile will be full
    rooms: [[Box<Room>; 8]; 8]

}
impl Floor {
    /*
    pub fn test_floor() -> Floor {
        // Initialize all grid spaces with None
        let rooms: [[Box<Room>; 8]; 8] = [[Box::<Room>::new(Room::non_room()); 8]; 8];

        Floor { rooms }
    }

     */
}
