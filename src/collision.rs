use crate::game::*;
use crate::util::*;
use crate::player::PowerUp;
use crate::entity::*;
use crate::tile::*;
use crate::menu::*;
use crate::blackboard::*;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use roguelike::SDLCore;
use crate::boxes::*;

pub fn enemy_collision(enemy: &mut Enemy, x: &i32, y: &i32) {


    let intersection = enemy.box_es.get_walkbox(enemy.pos).intersection(Rect::new(
        LEFT_WALL + x * TILE_WIDTH,
        TOP_WALL + y * TILE_WIDTH,
        TILE_WIDTH as u32,
        TILE_WIDTH as u32
    ));

    let inter_rect = match intersection {
        Some(x) => x,
        None => return,
    };

    //println!{"Collision"};

    let mut x_offset = inter_rect.width() as i32;
    let mut y_offset = inter_rect.height() as i32;

    if enemy.pos.x < inter_rect.x() as f32 && //To left
    enemy.pos.y < inter_rect.y() as f32 { //And Above
        match enemy.dir{
            Direction::Down | Direction::Up => { //Act like it is only to the left
                y_offset = 0;
            }
            Direction::Right | Direction::Left => { //Act like it is only above
                x_offset = 0;
            }
            _ => {
                println!("What? This shouldn't be happening");
            }
        }
    }else if enemy.pos.x < inter_rect.x() as f32 && //To left
    enemy.pos.y > (inter_rect.y() + inter_rect.height() as i32) as f32{ //And below
        match enemy.dir{
            Direction::Up | Direction::Down=> { //Act like it is only to left
                y_offset = 0;
            }
            Direction::Right | Direction::Left => { //Act like it is only below
                x_offset = 0;
                y_offset *= -1;
            }
            _ => {
                println!("What? I think I left the stove on");
            }
        }
    }else if enemy.pos.x > (inter_rect.x() + inter_rect.width() as i32) as f32  &&//To right
    enemy.pos.y < inter_rect.y() as f32 { //And above
        match enemy.dir{
            Direction::Down | Direction::Up=> { //Act like it is only to right
                x_offset *= -1;
                y_offset = 0;
            }
            Direction::Left | Direction::Right => { //Act like it is only above
                x_offset = 0;
            }
            _ => {
                println!("What? Is life even real?");
            }
        }
    }else if enemy.pos.x > (inter_rect.x() + inter_rect.width() as i32) as f32 &&//To right
    enemy.pos.y > (inter_rect.y() + inter_rect.height() as i32) as f32{ //And below
        match enemy.dir{
            Direction::Up|Direction::Down => { //Act like it is only to right
                x_offset *= -1;
                y_offset = 0;
            }
            Direction::Left | Direction::Right => { //Act like it is only below
                x_offset = 0;
                y_offset *= -1;
            }
            _ => {
                println!("What? I think I might be in hell");
            }
        }

    }else if enemy.pos.x < inter_rect.x() as f32 {
        // TO THE LEFT OF ROCK
        y_offset = 0;
    }else if enemy.pos.x > (inter_rect.x() + inter_rect.width() as i32) as f32 {
        // TO THE RIGHT OF ROCK
        x_offset *= -1;
        y_offset = 0;
    }else if enemy.pos.y < inter_rect.y() as f32 {
        // ABOVE ROCK
        x_offset = 0;
    }else if enemy.pos.y > (inter_rect.y() + inter_rect.height() as i32) as f32 {
        // BELOW ROCK
        x_offset = 0;
        y_offset *= -1;
    }

    //println!("Offset for X BEFORE: {}, for Y: {}", x_offset, y_offset);
    enemy.pos.x -= x_offset as f32;
    enemy.pos.y -= y_offset as f32;
}

pub fn base(game : &mut Game, core : &mut SDLCore, menu : &mut MenuState, blackboard: &BlackBoard) {
// Outermost wall collision
        game.player.pos.x = game.player.pos.x.clamp(
            LEFT_WALL as f32 + (game.player.box_es.walkbox.x/2) as f32,
            RIGHT_WALL as f32 - (game.player.box_es.walkbox.x/2) as f32
        );
        game.player.pos.y = game.player.pos.y.clamp(
            TOP_WALL as f32 + (game.player.box_es.walkbox.y/2) as f32,
            BOT_WALL as f32 - (game.player.box_es.walkbox.y/2) as f32
        );

    // More robust clamping for walls
        if game.player.pos.y < TOP_WALL as f32 + 5.0 * 64.0 || game.player.pos.y > BOT_WALL as f32 - 5.0 * 64.0 {
            game.player.pos.x = game.player.pos.x.clamp(
                LEFT_WALL as f32 + 64.0 + (game.player.box_es.walkbox.x/2) as f32,
                RIGHT_WALL as f32 - 64.0 - (game.player.box_es.walkbox.x/2) as f32
            );
        }
    if game.player.pos.x < LEFT_WALL as f32 + 8.0 * 64.0 || game.player.pos.x > RIGHT_WALL as f32 - 8.0 * 64.0 {
        game.player.pos.y = game.player.pos.y.clamp(
            TOP_WALL as f32 + 64.0 + (game.player.box_es.walkbox.x/2) as f32,
            BOT_WALL as f32 - 64.0 - (game.player.box_es.walkbox.x/2) as f32
        );
    }

    // Maintain enemy bounds for the room and check player collisions
        let mut enemy_list = game.current_room().enemies.clone();

        let mut live_count = 0;
        for enemy in enemy_list.iter_mut() {
            if enemy.death == false{
                live_count += 1;
            }
        }

        for enemy in enemy_list.iter_mut() {

            enemy.lastpos = enemy.pos; //Update the last position
            enemy.pos.x = enemy.pos.x.clamp(
                (LEFT_WALL as f32 + (enemy.box_es.walkbox.x * 4) as f32) - TILE_WIDTH as f32,
                (RIGHT_WALL as f32 - (enemy.box_es.walkbox.x * 4) as f32) + TILE_WIDTH as f32
            );
            enemy.pos.y = enemy.pos.y.clamp(
                (TOP_WALL as f32 + (enemy.box_es.walkbox.y * 4) as f32) - TILE_WIDTH as f32,
                (BOT_WALL as f32 - (enemy.box_es.walkbox.y * 4) as f32) + TILE_WIDTH as f32
            );

            let player_test = game.player.box_es.get_hitbox(game.player.pos);
            // If the test enemy is in the current room of the player...

            //handles two + enemies dying at once for power up, spawns random power up from enemy types in room
            if game.current_room().gemCount != 1 &&  BlackBoard::get_enemy_quantity(game) == 0 {
                game.current_room_mut().increment_gem();
                game.current_room_mut()
                    .tile_at(288, 100)
                    .place_gem(match enemy.kind {
                        EnemyKind::Health => Gem::Red,
                        EnemyKind::Speed => Gem::Blue,
                        EnemyKind::Attack => Gem::Yellow,
                        EnemyKind::Final => Gem::None,
                    });
            }


            // FINAL BOSS projectile (no it isn't, it works for all projectiles)
            //enemy.move_projectile(&game.current_room().tile_at(atk.pos.x, atk.pos.y));
            enemy.move_projectile();
            if !enemy.death() {


                //If enemy is attacking
                if enemy.recently_attacked() {
                    //See if player collides with attackbox
                    let enemy_attack = enemy.box_es.get_attackbox(enemy.pos, enemy.dir);
                    if player_test.has_intersection(enemy_attack) {
                        //Enemy attacked player
                        //game.player.take_damage(1, P_INVINCIBILITY_TIME);
                        game.player.take_damage(enemy.attack_damage(), P_INVINCIBILITY_TIME);
                        if game.player.death() {
                            *menu = MenuState::GameOver;
                        }
                    }
                }

                // If the test enemy's walkbox intersects with the player walkbox...
                let wb_test = enemy.box_es.get_hitbox(enemy.pos);
                // Attempt at collision with attackbox
                if game.player.is_attacking {
                    let player_attack = game.player.box_es.get_attackbox(game.player.pos, game.player.dir);
                    //let player_attack = game.player.get_attackbox_world();
                    if wb_test.has_intersection(player_attack) {
                        enemy.take_damage(game.player.attack, E_INVINCIBILITY_TIME);
                        //edge case for enemies dying for power up
                        if game.current_room().gemCount != 1 &&  BlackBoard::get_enemy_quantity(game) == 0 {
                            enemy.power = true;
                        }
                        //main case to determine power up
                        if enemy.death == true && (live_count == 1 || blackboard.boss_fight) {
                            enemy.power = true;
                            game.current_room_mut().increment_gem();
                        }
                        //executes if power up is true meaning a power up should be dropped as its the last enemy
                        if enemy.power == true {
                            // Place gem on enemy's current tile.
                            // TODO: Factor in walkability for tile that the gem drops on.
                            game.current_room_mut()
                                .tile_at(enemy.get_pos_x(), enemy.get_pos_y())
                                .place_gem(match enemy.kind {
                                    EnemyKind::Health => Gem::Red,
                                    EnemyKind::Speed => Gem::Blue,
                                    EnemyKind::Attack => Gem::Yellow,
                                    EnemyKind::Final => Gem::None,
                                });
                            enemy.power = false;
                        }
                    }
                }
                if game.player.using_bomb {
                    let player_bomb = game.player.box_es.get_bombbox(game.player.pos_static, game.player.dir);

                    // Used to blow up door on opposite side
                    let mut x_off = 0;
                    let mut y_off = 0;

                    // Blow up door
                    if Rect::new(LEFT_WALL + 0 * 64, TOP_WALL + 5 * 64, 64, 64).has_intersection(player_bomb) {
                        game.current_room_mut().tiles[5][0].explode();
                        x_off -= 1;
                    }
                    else if Rect::new(LEFT_WALL + 16 * 64, TOP_WALL + 5 * 64, 64, 64).has_intersection(player_bomb) {
                        game.current_room_mut().tiles[5][16].explode();
                        x_off += 1;
                    }
                    else if Rect::new(LEFT_WALL + 8 * 64, TOP_WALL + 0 * 64, 64, 64).has_intersection(player_bomb) {
                        game.current_room_mut().tiles[0][8].explode();
                        y_off -= 1;
                    }
                    else if Rect::new(LEFT_WALL + 8 * 64, TOP_WALL + 10 * 64, 64, 64).has_intersection(player_bomb) {
                        game.current_room_mut().tiles[10][8].explode();
                        y_off += 1;
                    }

                    let mut door_pos = Vec2::new(0, 0);
                    if x_off == -1 {
                        door_pos = Vec2::new(16, 5);
                    } else if x_off == 1 {
                        door_pos = Vec2::new(0, 5);
                    } else if y_off == - 1 {
                        door_pos = Vec2::new(8, 10);
                    } else if y_off == 1 {
                        door_pos = Vec2::new(8, 0);
                    }

                    game.map.floors[game.cf]
                        .rooms[(game.cr.y + y_off) as usize][(game.cr.x + x_off) as usize]
                        .tiles[door_pos.y as usize][door_pos.x as usize]
                        .explode();

                    if wb_test.has_intersection(player_bomb) {
                        //println!("Bomb collided with enemy!");
                        enemy.take_damage(4, E_INVINCIBILITY_TIME); //Bomb deals 3 damage
                        //println!("damage done was 3 from bomb");
                        if enemy.death == true && live_count == 1
                        {
                            enemy.power = true;
                        }
                        if enemy.power == true {
                            // Place gem on enemy's current tile.
                            // TODO: Factor in walkability for tile that the gem drops on.
                            game.current_room_mut()
                                .tile_at(enemy.get_pos_x(), enemy.get_pos_y())
                                .place_gem(match enemy.kind {
                                    EnemyKind::Health => Gem::Red,
                                    EnemyKind::Speed => Gem::Blue,
                                    EnemyKind::Attack => Gem::Yellow,
                                    EnemyKind::Final => Gem::None,
                                });

                            enemy.power = false;
                        }
                    }
                }


                // Then there's a collision!
                /*if wb_test.has_intersection(player_test) {
                    //Damage enemy also! For some reason
                    //enemy.damage(1);
                    // Update player invincibility window and take damage to the player.
                    // Parameters: 1 is the damage amount, 1750 is the amount of ms before the cooldown window expires
                    game.player.take_damage( ENEMY_INTERSECTION_DAMAGE, P_INVINCIBILITY_TIME );


                    // If the player is dead, update to the game over menu state
                    if game.player.death() {
                        *menu = MenuState::GameOver;
                    }
                }*/
            }

            for atk in &enemy.atk_list{
                // let cur = Vec2::new(
                //     (atk.pos.x as i32 - LEFT_WALL) / TILE_WIDTH,
                //     (atk.pos.y as i32 - TOP_WALL) / TILE_WIDTH
                // );
                //let current_tile = &mut game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tiles[((atk.pos.y as i32 - TOP_WALL) / TILE_WIDTH) as usize][((atk.pos.x as i32 - LEFT_WALL) / TILE_WIDTH) as usize];
                //let current_tile = &mut game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tile_at(atk.pos.x as i32, atk.pos.y as i32);
                //let current_tile = &mut game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tile_at(cur.x, cur.y);
                //println!("{:?}", current_tile.walkability());
                let wb_test = atk.box_es.get_hitbox(atk.pos);
                let player_test = game.player.box_es.get_hitbox(game.player.pos);

                if wb_test.has_intersection(player_test){
                    game.player.take_damage(atk.damage, P_INVINCIBILITY_TIME);
                    if game.player.death() {
                        *menu = MenuState::GameOver;
                    }
                }
            }
        }




        core.wincan.set_draw_color(Color::RGBA(128, 0, 0, 255));
        let mut x = 0;
        let mut y = 0;
        // This can't be done with the current room function bc it returns a reference which messes up internal stuff
        for row in &game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tiles {
            for t in row {
                match t.walkability() {
                    Walkability::Wall | Walkability::Rock | Walkability::Pit => {
                        // Hacky af block collision that needs to be changed later
                        let opt = game.player.box_es.get_walkbox(game.player.pos).intersection(Rect::new(
                            LEFT_WALL + x * TILE_WIDTH,
                            TOP_WALL + y * TILE_WIDTH,
                            TILE_WIDTH as u32,
                            TILE_WIDTH as u32
                        ));
                        for enemy in enemy_list.iter_mut() {
                            match enemy.kind {
                                EnemyKind::Speed => {},
                                _ => enemy_collision(enemy, &x, &y)
                            }
                        }
                        // increment x
                        // if we do this later it messes thing up due to the continue statement in
                        // the unboxing
                        x += 1;

                        let inter_rect = match opt {
                            Some(x) => x,
                            None => continue, // If no intersection just leave function, we're done
                        };
                        let mut x_offset = inter_rect.width() as i32;
                        let mut y_offset = inter_rect.height() as i32;

                        if game.player.pos.x < inter_rect.x() as f32 {
                            // TO THE LEFT OF ROCK
                            y_offset = 0;
                        }
                        if game.player.pos.x > (inter_rect.x() + inter_rect.width() as i32) as f32 {
                            // TO THE RIGHT OF ROCK
                            x_offset *= -1;
                            y_offset = 0;
                        }
                        if game.player.pos.y < inter_rect.y() as f32 {
                            // ABOVE ROCK
                            x_offset = 0;
                        }
                        if game.player.pos.y > (inter_rect.y() + inter_rect.height() as i32) as f32 {
                            // BELOW ROCK
                            x_offset = 0;
                            y_offset *= -1;
                        }

                        game.player.pos.x -= x_offset as f32;
                        game.player.pos.y -= y_offset as f32;
                    }

                    _ => x += 1,
                }
            }

            // Prepare for next iteration of loop
            y += 1;
            x = 0;
        }

        game.current_room_mut().enemies = enemy_list;
    }
