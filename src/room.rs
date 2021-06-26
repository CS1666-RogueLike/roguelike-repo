
use crate::tile::*;

pub const ROOM_WIDTH: i32 = 17;
pub const ROOM_HEIGHT: i32 = 11;

pub struct Room {
    pub tiles: Vec<Vec<Box<dyn Tile>>>,
}

/*
 *  IMPORTANT NOTE ABOUT TRAITS
 *
 *  For a structure like a vec! to work properly, it needs to know the exact size of the objects
 *  it's containing. However, traits are simply interfaces implemented by types, and thus objects
 *  can be multiple sizes. Because their size is therefore unknown, they can't be stored in
 *  containers alone. Instead, we have to allocate them to the heap using a Box (a type that is of
 *  a consistent size), then act on things through the box.
 */

impl Room {
    // Returns a room that the developer sets every tile of manually.
    pub fn new_test_room() -> Room {

        // ----------------------- READ THIS!!!!!!!!!!!!!!!!! -----------------------
        // Manually defining the room array is needed, but the syntax to do that manually would be a mess.
        // Instead, we can define an array of the same size of just characters, with each character
        // representing a type of tile. Then, we translate that tile from this easy to view setup
        // to an actual room array filled with tiles.

        // KEY:
        // _ -> Ground (to make looking at it easier)
        // w -> Wall
        // r -> Rock
        // p -> Pit

        let blueprint = [
        //                                   MID
        //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','_','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','R','_','R','_','_','_','_','_','R','R','R','R','R','_','W'], // 2
            ['W','_','R','_','R','_','_','_','_','_','R','_','_','_','R','_','W'], // 3
            ['W','_','R','_','R','_','R','_','_','_','R','_','P','_','R','_','W'], // 4
            ['_','_','R','R','R','_','_','_','_','_','_','_','P','_','R','_','_'], // 5 MID
            ['W','_','R','_','R','_','R','_','_','_','R','_','P','_','R','_','W'], // 6
            ['W','_','R','_','R','_','R','_','_','_','R','_','_','_','R','_','W'], // 7
            ['W','_','R','_','R','_','R','_','_','_','R','R','R','R','R','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','_','W','W','W','W','W','W','W','W'], // 10
        ];

        // Vec that contains actual Tile trait implementing structs
        let mut tiles: Vec<Vec<Box<dyn Tile>>> = Vec::new();

        for y in 0..ROOM_HEIGHT {
            // Add a row to our struct
            tiles.push(Vec::new());
            for x in 0..ROOM_WIDTH {
                match blueprint[y as usize][x as usize] {

                    // These have to be in boxes because the compiler does know how big the
                    // implementations of the trait are. Because a box is essentially a pointer,
                    // it's of a size the compiler knows about. Thus, we give it a pointer to an
                    // implementation of the type, and the compiler is satisfied.
                    '_' => tiles[y as usize].push(Box::new(Ground {})),
                    'W' => tiles[y as usize].push(Box::new(Wall {})),
                    'R' => tiles[y as usize].push(Box::new(Rock {})),
                    'P' => tiles[y as usize].push(Box::new(Pit {})),

                    _ => panic!("NO MATCH FOR TILE TYPE"), // NOTE THAT THIS IS DIFFERENT FROM '_' WHICH CHECKS FOR THE UNDERSCORE CHARACTER
                    // This needs to panic if an unrecogized character is found,
                    // otherwise the rooms won't be the right size and a bunch
                    // of crazy buggy stuff could happen.
                }
            }
        }

        /*
        // Debug print to make sure structure was built properly
        println!(" -------- FROM ACTUAL TRAIT STORING ARRAY -------");
        for row in &tiles {
            for t in row {
                match t.walkability() {
                    Walkability::Floor => print!(" _ "),
                    Walkability::Wall => print!(" W "),
                    Walkability::Rock => print!(" R "),

                    _ => panic!("NO MATCH FOR TILE TYPE"),
                    // This needs to panic, otherwise the rooms won't be the right size and a bunch
                    // of crazy buggy stuff could happen.

                }
            }
            println!();
        }
        */

        // Return room struct.
        Room {
            tiles: tiles,
        }


    }
}
