use crate::room::*;
use crate::entity::*;
use crate::util::*;
use crate::tile::*;
//use crate::procgen::*;
use rand::Rng;


use crate::procgen::RecursiveBacktracker;


pub struct Floor {

    // This is an option because not every tile will be full
    pub rooms: Vec<Vec<Box<Room>>>

}
impl Floor {

    pub fn boss_floor() -> Floor {
        let room = [
            //                                  MID
            //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 0
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 3
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 5 MID
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 7
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
            ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','Q','W'], // 9
            ['W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W','W'], // 10
        ];

        // Initialize all grid spaces with None
        let mut rooms: Vec<Vec<Box<Room>>> = Vec::with_capacity(8);

        // Fill floor with invalid roooms
        for x in 0..8 {
            rooms.push(Vec::with_capacity(8));
            for _y in 0..8 {
                rooms[x].push(Box::new(Room::non_room()));
            }
        }

        // FOR FINAL BOSS TESTING ONLY
        rooms[START_Y as usize][START_X as usize] = Box::new(Room::new_test_room(room));
        let mut enemies = Vec::new();
        enemies.push(Enemy::new(Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 2 * 64) as f32 + 40.0), EnemyKind::Final));
        rooms[START_Y as usize][START_X as usize].add_enemies(enemies);
        // END FINAL BOSS
        Floor { rooms }
    }

    pub fn gen_floor() -> Floor {
        let mut rng = rand::thread_rng();
        let start_room = [
                //                                  MID
                //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
                ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
                ['W','_','R','R','_','_','_','_','_','_','_','_','_','R','R','_','W'], // 2
                ['W','_','R','R','_','R','_','_','_','_','_','R','_','R','R','_','W'], // 3
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 4
                ['D','_','_','_','_','_','_','_','_','Q','_','_','_','_','_','_','D'], // 5 MID
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 6
                ['W','_','R','R','_','R','_','_','_','_','_','R','_','R','R','_','W'], // 7
                ['W','_','R','R','_','_','_','_','_','_','_','_','_','R','R','_','W'], // 8
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
                ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        let end_room = [
        //                                 MID
    //      //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
                ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
                ['W','_','R','R','R','R','R','_','_','_','R','R','R','R','R','_','W'], // 2
                ['W','_','R','_','_','_','_','_','_','_','_','_','_','_','R','_','W'], // 3
                ['W','_','R','_','_','_','_','_','_','_','_','_','_','_','R','_','W'], // 4
                ['D','_','_','_','_','_','_','_','T','_','_','_','_','_','_','_','D'], // 5 MID
                ['W','_','R','_','_','_','_','_','_','_','_','_','_','_','R','_','W'], // 6
                ['W','_','R','_','_','_','_','_','_','_','_','_','_','_','R','_','W'], // 7
                ['W','_','R','R','R','R','R','_','_','_','R','R','R','R','R','_','W'], // 8
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
                ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];
        let key_room = [
        //         //                                   MID
    //      //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
                ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 0
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 1
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 2
                ['W','_','_','R','R','R','R','R','_','R','R','R','R','R','_','_','W'], // 3
                ['W','_','_','R','_','_','_','_','_','_','_','_','_','R','_','_','W'], // 4
                ['D','_','_','_','_','_','_','_','K','_','_','_','_','_','_','_','D'], // 5 MID
                ['W','_','_','R','_','_','_','_','_','_','_','_','_','R','_','_','W'], // 6
                ['W','_','_','R','R','R','R','R','_','R','R','R','R','R','_','_','W'], // 7
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 8
                ['W','_','_','_','_','_','_','_','_','_','_','_','_','_','_','_','W'], // 9
                ['W','W','W','W','W','W','W','W','D','W','W','W','W','W','W','W','W'], // 10
        ];


        // Initialize all grid spaces with None
        let mut rooms: Vec<Vec<Box<Room>>> = Vec::with_capacity(8);

        // Fill floor with invalid roooms
        for x in 0..8 {
            rooms.push(Vec::with_capacity(8));
            for _y in 0..8 {
                rooms[x].push(Box::new(Room::non_room()));
            }
        }
        let rb = RecursiveBacktracker::new(Some(8));
        let procgen_res = rb.run();
        let random_num = rng.gen_range(2..=6);
        for cord in procgen_res.iter() {
            let mut blueprint = cellular_automata();
            if cord.1 == START_X && cord.0 == START_Y
            {
                blueprint = start_room;
            }
            if cord == procgen_res.last().unwrap()
            {
                blueprint = end_room;
            }
            if cord == procgen_res.get(random_num).unwrap()
            {
                blueprint = key_room;
            }
            rooms[cord.1 as usize][cord.0 as usize] = Box::new(Room::new_test_room(blueprint));
        }

        for (x,y) in procgen_res.iter() {
            let dy = *y;
            let dx = *x;
            if dy > 0 {
                if !rooms[(dy-1) as usize][dx as usize].exists {
                    let current_room = &mut rooms[dy as usize][dx as usize];
                    current_room.tiles[0][8] = Box::new(Wall{});// as &dyn Tile;
                }
            }
            if dx > 0 {
                if !rooms[dy as usize][(dx-1) as usize].exists {
                    let current_room = &mut rooms[dy as usize][dx as usize];
                    current_room.tiles[5][0] = Box::new(Wall{});// as &dyn Tile;
                }
            }
            if dy < 7 {
                if !rooms[(dy + 1) as usize][dx as usize].exists {
                    let current_room = &mut rooms[dy as usize][dx as usize];
                    current_room.tiles[10][8] = Box::new(Wall{});// as &dyn Tile;
                }
            }
            if dx < 7 {
                if !rooms[dy as usize][(dx+1) as usize].exists {
                    let current_room = &mut rooms[dy as usize][dx as usize];
                    current_room.tiles[5][16] = Box::new(Wall{});// as &dyn Tile;
                }
            }
            if dx == 0 {
                let current_room = &mut rooms[dy as usize][dx as usize];
                current_room.tiles[5][0] = Box::new(Wall{});// as &dyn Tile;
            }
            if dy == 0 {
                let current_room = &mut rooms[dy as usize][dx as usize];
                current_room.tiles[0][8] = Box::new(Wall{});// as &dyn Tile;
            }
            if dx == 7 {
                let current_room = &mut rooms[dy as usize][dx as usize];
                current_room.tiles[5][16] = Box::new(Wall{});// as &dyn Tile;
            }
            if dy == 7 {
                let current_room = &mut rooms[dy as usize][dx as usize];
                current_room.tiles[10][8] = Box::new(Wall{});// as &dyn Tile;
            }
            if dx == START_X && dy == START_Y
            {
                continue;
            }

            let mut enemies = Vec::new();
            let current_room = &mut rooms[dy as usize][dx as usize];
            let num_enemies = rng.gen_range(0 .. 4);
            for _i in 0 ..= num_enemies {
                let boundary_x = rng.gen_range(3 ..= 13);
                let boundary_y = rng.gen_range(2 ..= 8);
                let enemy_rand: EnemyKind = match rng.gen_range(0 .. 3) {
                    0 => EnemyKind::Attack,
                    1 => EnemyKind::Health,
                    2 => EnemyKind::Speed,
                    _ => EnemyKind::Health,
                };
                enemies.push(Enemy::new( Vec2::new((LEFT_WALL + boundary_x * 64) as f32 + 32.0, (TOP_WALL + boundary_y * 64) as f32 + 40.0), enemy_rand));
            }
            current_room.add_enemies(enemies);
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
        // Q -> bomb


        //messing with cellular automata
        // double for loop that starts 1 out from each wall.  we have each permutation inspect its neighborhood
        //which will consist of its 8 surrounding blocks.  It will match its neighborhood with a preset
        //ruleset that will determine if it is a rock or not(aka a 1 or 0).  There will be an initial layout to
        //determine the specific room, the layout will be iterated over while checking all neigbors for each
        //iteration.  This will store a result in a seperate array that will be the final rock placement array.
        fn cellular_automata() -> [[char; 17]; 11]
        {
            let mut read = [
                // 0
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 0
                ['_', '_', '_', 'R', '_', '_', '_', 'R', '_', 'R', '_', '_', '_', '_', '_'], // 1
                ['_', '_', 'R', '_', '_', 'R', '_', '_', '_', '_', 'R', 'R', 'R', '_', '_'], // 2
                ['_', '_', 'R', '_', '_', '_', '_', 'R', '_', 'R', '_', '_', '_', '_', '_'], // 3
                ['_', '_', '_', 'R', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 4 MID
                ['_', '_', '_', '_', '_', '_', '_', 'R', '_', 'R', '_', 'R', '_', '_', '_'], // 5
                ['_', '_', 'R', '_', 'R', '_', '_', 'R', '_', 'R', '_', '_', 'R', 'R', '_'], // 6
                ['_', '_', '_', 'R', '_', 'R', '_', 'R', '_', '_', '_', '_', 'R', '_', '_'], // 7
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 8
            ];
            let mut write_vec = [
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 0
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', 'R', '_', '_', '_', '_', '_'], // 1
                ['_', '_', 'R', '_', '_', '_', '_', '_', '_', '_', '_', 'R', '_', '_', '_'], // 2
                ['_', '_', '_', '_', '_', '_', '_', 'R', '_', '_', '_', '_', '_', '_', '_'], // 3
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 4 MID
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 5
                ['_', '_', '_', '_', 'R', '_', '_', 'R', '_', '_', '_', '_', '_', '_', '_'], // 6
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'R', '_', '_'], // 7
                ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 8
            ];

            let mut count = 0;
            let mut rng = rand::thread_rng();
            while count < 100 {
                for rows in 1..8 {
                    for cols in 1..14 {
                        let mut blank = 0;
                        let mut rock = 0;
                        let mut spike = 0;
                        let mut pit = 0;
                        let mut neighborhood = Vec::new();
                        neighborhood.push(read[rows - 1][cols - 1]);
                        neighborhood.push(read[rows - 1][cols]);
                        neighborhood.push(read[rows - 1][cols + 1]);
                        neighborhood.push(read[rows][cols - 1]);
                        neighborhood.push(read[rows][cols]);
                        neighborhood.push(read[rows][cols + 1]);
                        neighborhood.push(read[rows + 1][cols - 1]);
                        neighborhood.push(read[rows + 1][cols]);
                        neighborhood.push(read[rows + 1][cols + 1]);
                        for i in neighborhood {
                            match i {
                                '_' => blank += 1,
                                'R' => rock += 1,
                                'S' => spike += 1,
                                _ => blank += 1,
                            };
                        }
                        if blank > 8 {
                            let random_num = rng.gen_range(0..=2);
                            if random_num == 0 {
                                write_vec[rows][cols] = 'P';
                            } else {
                                write_vec[rows][cols] = '_';
                            }
                        } else if blank > 7 {
                            let random_num = rng.gen_range(0..=2);
                            if random_num == 0 {
                                write_vec[rows][cols] = 'R';
                            } else {
                                write_vec[rows][cols] = '_';
                            }
                        } else if rock + pit > 2 {
                            write_vec[rows][cols] = 'S';
                        }
                        if spike > 2 {
                            write_vec[rows][cols] = '_';
                        }
                        if rock > 4 {
                            write_vec[rows][cols] = '_';
                        }
                        if pit > 4 {
                            write_vec[rows][cols] = '_';
                        }
                        if (rock + pit + spike) > 5 {
                            write_vec[rows][cols] = '_';
                        }
                    }
                }
                read = write_vec.clone();
                count += 1;
            }
            let mut blueprint = [
                //                                   MID
                //    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
                ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'D', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 0
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'W'], // 1
                ['W', '_', 'R', 'R', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'R', 'R', '_', 'W'], // 2
                ['W', '_', 'R', 'R', '_', 'R', '_', '_', '_', '_', '_', 'R', '_', 'R', 'R', '_', 'W'], // 3
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'W'], // 4
                ['D', '_', 'Q', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'D'], // 5 MID
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'W'], // 6
                ['W', '_', 'R', 'R', '_', 'R', '_', 'S', 'S', 'S', '_', 'R', '_', 'R', 'R', '_', 'W'], // 7
                ['W', '_', 'R', 'R', '_', '_', '_', 'S', 'S', 'S', '_', '_', '_', 'R', 'R', '_', 'W'], // 8
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'W'], // 9
                ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'D', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 10
            ];
            for rows in 2..9 {
                for cols in 2..15 {
                    blueprint[rows][cols] = write_vec[rows - 1][cols - 1];
                }
            }
            return blueprint;
        }

        Floor { rooms }
    }
}
