use crate::room::*;
use crate::entity::*;
use crate::util::*;

pub struct Floor {

    // This is an option because not every tile will be full
    pub rooms: Vec<Vec<Box<Room>>>

}
impl Floor {

    pub fn test_floor() -> Floor {
        // Initialize all grid spaces with None
        let mut rooms: Vec<Vec<Box<Room>>> = Vec::with_capacity(8);

        // Fill floor with invalid roooms
        for x in 0..8 {
            rooms.push(Vec::with_capacity(8));
            for _y in 0..8 {
                rooms[x].push(Box::new(Room::non_room()));
            }
        }

        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // !!!!!!!!!! NOTE!: INDEXING SHOULD BE DONE AS [y][x] !!!!!!!!
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

        // Replace rooms with our custom setup

        // ----------------------- READ THIS!!!!!!!!!!!!!!!!! -----------------------
        // Manually defining the room array is needed, but the syntax to do that manually would be a mess.
        // Instead, we can define an array of the same size of just characters, with each character
        // representing a type of tile. Then, we translate that tile from this easy to view setup
        // to an actual room array filled with tiles.

        // KEY:
        // _ -> Ground (to make looking at it easier)
        // W -> Wall
        // R -> Rock
        // P -> Pit
        // D -> Door
        // S -> Spike
        // r -> red gem
        // y -> yellow gem
        // b -> blue gem

        // TOP RIGHT ROOM
        let mut blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','P','P','P','P','P','P','P','P','P','P','P','_','_','W'], // 3
            ['W','_','_','P','_','_','_','_','_','_','_','_','_','P','_','_','W'], // 4
            ['D','_','_','P','_','_','_','_','R','_','_','_','_','P','_','_','W'], // 5 MID
            ['W','_','_','P','_','_','_','_','_','_','_','_','_','P','_','_','W'], // 6
            ['W','_','_','P','P','P','P','P','_','P','P','P','P','P','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[3][4] = Box::new(Room::new_test_room(blueprint));
        let mut enemies_34_fl0 = Vec::new();
        enemies_34_fl0.push(Enemy::new(Vec2::new((LEFT_WALL + 12 * 64) as f32 + 32.0, (TOP_WALL + 7 * 64) as f32 + 40.0), EnemyKind::Speed));
        rooms[3][4].add_enemies(enemies_34_fl0);

        // TOP LEFT ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','R','_','_','_','_','_','_','_','_','_','R','_','_','W'], // 2
            ['W','_','R','R','R','_','_','_','_','_','_','_','R','R','R','_','W'], // 3
            ['W','_','_','R','_','_','P','P','P','P','P','_','_','R','_','_','W'], // 4
            ['D','_','_','_','_','_','P','P','P','P','P','_','_','_','_','_','D'], // 5 MID
            ['W','_','_','R','_','_','P','P','P','P','P','_','_','R','_','_','W'], // 6
            ['W','_','R','R','R','_','_','_','_','_','_','_','R','R','R','_','W'], // 7
            ['W','_','_','R','_','_','_','_','_','_','_','_','_','R','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[3][3] = Box::new(Room::new_test_room(blueprint));

        // TOP LEFT ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','R','R','R','R','R','R','R','R','R','R','R','_','_','W'], // 3
            ['W','_','_','R','_','_','_','_','_','_','_','_','_','R','_','_','W'], // 4
            ['W','_','_','R','_','_','_','_','K','_','_','_','_','R','_','_','D'], // 5 MID
            ['W','_','_','R','_','_','_','_','_','_','_','_','_','R','_','_','W'], // 6
            ['W','_','_','R','R','R','R','R','_','R','R','R','R','R','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[3][2] = Box::new(Room::new_test_room(blueprint));
        let mut enemies_32_fl0 = Vec::new();
        enemies_32_fl0.push(Enemy::new( Vec2::new((LEFT_WALL + 1 * 64) as f32 + 32.0, (TOP_WALL + 6 * 64) as f32 + 40.0), EnemyKind::Attack));
        rooms[3][2].add_enemies(enemies_32_fl0);


        // LEFT MID ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','P','P','W'], // 1
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','P','P','W'], // 2
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','P','P','W'], // 3
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','R','R','W'], // 4
            ['W','P','P','R','_','_','R','r','y','b','R','_','_','_','_','_','D'], // 5 MID
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','R','R','W'], // 6
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','P','P','W'], // 7
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','P','P','W'], // 8
            ['W','P','P','R','_','_','_','_','_','_','_','_','_','R','P','P','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[4][2] = Box::new(Room::new_test_room(blueprint));

        // MID ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','R','R','_','_','_','_','_','_','_','_','_','R','R','_','W'], // 2
            ['W','_','R','R','_','R','_','_','_','_','_','R','_','R','R','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['D','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','D'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','R','R','_','R','_','S','S','S','_','R','_','R','R','_','W'], // 7
            ['W','_','R','R','_','_','_','S','S','S','_','_','_','R','R','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[4][3] = Box::new(Room::new_test_room(blueprint));
        let mut enemies_43_fl0 = Vec::new();
        enemies_43_fl0.push(Enemy::new( Vec2::new((LEFT_WALL + 1 * 64) as f32 + 32.0, (TOP_WALL + 6 * 64) as f32 + 40.0), EnemyKind::Attack));
        enemies_43_fl0.push(Enemy::new(Vec2::new((LEFT_WALL + 12 * 64) as f32 + 32.0, (TOP_WALL + 7 * 64) as f32 + 40.0), EnemyKind::Attack));
        enemies_43_fl0.push(Enemy::new(Vec2::new((LEFT_WALL + 12 * 64) as f32 + 32.0, (TOP_WALL + 8 * 64) as f32 + 40.0), EnemyKind::Speed));
        rooms[4][3].add_enemies(enemies_43_fl0);
        
        // RIGHT MID ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','R','_','_','_','R','_','_','_','_','W'], // 1
            ['W','_','R','R','R','_','_','R','_','_','_','_','_','R','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','R','_','R','W'], // 3
            ['W','_','_','_','_','_','_','R','R','R','_','_','_','R','_','_','W'], // 4
            ['D','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 5 MID
            ['W','_','_','_','R','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','R','_','R','R','R','_','_','R','_','R','R','R','W'], // 7
            ['W','R','R','_','R','_','_','_','_','_','_','R','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','R','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[4][4] = Box::new(Room::new_test_room(blueprint));


        // -------------- BOTTOM ROW ------------------

        // LEFT BOT ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','R','R','R','R','R','R','_','_','_','_','R','_','_','W'], // 3
            ['W','_','_','R','_','_','_','_','R','_','P','P','_','R','_','_','W'], // 4
            ['W','_','_','R','_','P','P','_','R','_','P','P','_','R','_','_','D'], // 5 MID
            ['W','_','_','R','_','P','P','_','R','_','_','_','_','R','_','_','W'], // 6
            ['W','_','_','R','_','_','_','_','R','R','R','R','R','R','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[5][2] = Box::new(Room::new_test_room(blueprint));

        // BOT MID ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','P','P','P','P','P','R','_','_','_','R','P','P','P','P','P','W'], // 1
            ['W','P','R','R','R','R','R','_','_','_','R','R','R','R','R','P','W'], // 2
            ['W','P','R','_','_','_','_','_','_','_','_','_','_','_','R','P','W'], // 3
            ['W','R','R','_','_','_','_','_','_','_','_','_','_','_','R','R','W'], // 4
            ['D','_','_','_','_','_','_','_','T','_','_','_','_','_','_','_','D'], // 5 MID
            ['W','R','R','_','_','_','_','_','_','_','_','_','_','_','R','R','W'], // 6
            ['W','P','R','_','_','_','_','_','_','_','_','_','_','_','R','P','W'], // 7
            ['W','P','R','R','R','R','R','_','_','_','R','R','R','R','R','P','W'], // 8
            ['W','P','P','P','P','P','R','_','_','_','R','P','P','P','P','P','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[5][3] = Box::new(Room::new_test_room(blueprint));

        // RIGHT BOT ROOM
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','R','R','R','_','_','_','_','_','_','_','_','_','_','_','R','W'], // 1
            ['W','R','_','R','_','_','R','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','R','R','R','_','_','_','_','_','R','_','R','R','R','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','R','_','R','_','_','W'], // 4
            ['D','_','_','R','_','R','R','R','_','_','_','R','R','R','_','_','W'], // 5 MID
            ['W','_','_','_','_','R','_','R','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','R','R','R','_','_','R','_','_','_','_','_','W'], // 7
            ['W','_','R','_','_','_','_','_','_','_','_','_','_','_','R','_','W'], // 8
            ['W','_','_','_','_','_','_','_','R','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[5][4] = Box::new(Room::new_test_room(blueprint));


        // Print layout of rooms (debug)
        // println!();
        // println!();
        // for x in 0..8 {
        //     for y in 0..8 {
        //         print!("{} ", if rooms[x][y].exists {"R"} else {"_"});

        //     }
        //     println!();
        // }



        Floor { rooms }
    }

    pub fn test_floor_2() -> Floor {
        // Initialize all grid spaces with None
        let mut rooms: Vec<Vec<Box<Room>>> = Vec::with_capacity(8);

        // Fill floor with invalid roooms
        for x in 0..8 {
            rooms.push(Vec::with_capacity(8));
            for _y in 0..8 {
                rooms[x].push(Box::new(Room::non_room()));
            }
        }

        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // !!!!!!!!!! NOTE!: INDEXING SHOULD BE DONE AS [y][x] !!!!!!!!
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

        // Replace rooms with our custom setup

        // ----------------------- READ THIS!!!!!!!!!!!!!!!!! -----------------------
        // Manually defining the room array is needed, but the syntax to do that manually would be a mess.
        // Instead, we can define an array of the same size of just characters, with each character
        // representing a type of tile. Then, we translate that tile from this easy to view setup
        // to an actual room array filled with tiles.

        // KEY:
        // _ -> Ground (to make looking at it easier)
        // W -> Wall
        // R -> Rock
        // P -> Pit
        // D -> Door
        // S -> Spike

        // MID ROOM
        let mut blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','R','R','R','R','R','R','_','R','R','R','R','R','R','_','W'], // 2
            ['W','_','R','R','R','R','R','R','_','R','R','R','R','R','R','_','W'], // 3
            ['W','_','R','R','_','_','_','_','_','_','_','_','_','R','R','_','W'], // 4
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','D'], // 5 MID
            ['W','_','R','R','_','_','_','_','_','_','_','_','_','R','R','_','W'], // 6
            ['W','_','R','R','R','R','R','R','_','R','R','R','R','R','R','_','W'], // 7
            ['W','_','R','R','R','R','R','R','_','R','R','R','R','R','R','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[4][3] = Box::new(Room::new_test_room(blueprint));

        // ABOVE
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['D','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[3][3] = Box::new(Room::new_test_room(blueprint));

        // 2 ABOVE
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','D'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[2][3] = Box::new(Room::new_test_room(blueprint));

        // 2 ABOVE 1 RIGHT
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['D','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','D'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[2][4] = Box::new(Room::new_test_room(blueprint));

        // 2 ABOVE 2 RIGHT
        blueprint = [
            //                                   MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['D','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[2][5] = Box::new(Room::new_test_room(blueprint));

        // 1 ABOVE 2 RIGHT
        blueprint = [
            //                               MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[3][5] = Box::new(Room::new_test_room(blueprint));

        // 0 ABOVE 2 RIGHT
        blueprint = [
            //                               MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['D','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[4][5] = Box::new(Room::new_test_room(blueprint));

        // 0 ABOVE 1 RIGHT
        blueprint = [
            //                               MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['D','_','_','_','_','_','_','_','K','_','_','_','_','_','_','_','D'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[4][4] = Box::new(Room::new_test_room(blueprint));
        let mut enemies_44_fl1 = Vec::new();
        enemies_44_fl1.push(Enemy::new(Vec2::new((LEFT_WALL + 14 * 64) as f32 + 32.0, (TOP_WALL + 9 * 64) as f32 + 40.0), EnemyKind::Health));
        rooms[4][4].add_enemies(enemies_44_fl1);

        // 1 ABOVE 1 LEFT
        blueprint = [
            //                               MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['W','_','_','_','_','_','_','_','T','_','_','_','_','_','_','_','D'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];
        rooms[3][2] = Box::new(Room::new_test_room(blueprint));





        // -------------- BOTTOM ROW ------------------


        // Print layout of rooms (debug)
        // println!();
        // println!();
        // for x in 0..8 {
        //     for y in 0..8 {
        //         print!("{} ", if rooms[x][y].exists {"R"} else {"_"});

        //     }
        //     println!();
        // }

        Floor { rooms }
    }
}
