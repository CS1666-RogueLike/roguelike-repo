use crate::game::*;
use crate::util::*;
use crate::player::PowerUp;
use crate::entity::*;
use crate::tile::*;
use crate::menu::*;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use roguelike::SDLCore;

pub fn base(mut game : &mut Game, mut core : &mut SDLCore, mut menu : &mut MenuState){
// Outermost wall collision
        game.player.pos.x = game.player.pos.x.clamp(LEFT_WALL as f32 + (game.player.walkbox.x/2) as f32, RIGHT_WALL as f32 - (game.player.walkbox.x/2) as f32);
        game.player.pos.y = game.player.pos.y.clamp(TOP_WALL as f32 + (game.player.walkbox.y/2) as f32, BOT_WALL as f32 - (game.player.walkbox.y/2) as f32);

        // TODO: Goal is to generalize hitbox data into a trait so that we can condense logic

        // Maintain enemy bounds for the room and check player collisions
        let mut enemy_list = game.current_room().enemies.clone();
                    
        for enemy in enemy_list.iter_mut() {
            enemy.pos.x = enemy.pos.x.clamp(LEFT_WALL as f32 + (enemy.walkbox.x * 4) as f32, RIGHT_WALL as f32 - (enemy.walkbox.x * 4) as f32);
            enemy.pos.y = enemy.pos.y.clamp(TOP_WALL as f32 + (enemy.walkbox.y * 4) as f32, BOT_WALL as f32 - (enemy.walkbox.y * 4) as f32);

            // If the test enemy is in the current room of the player...
            if !enemy.death() {
                // If the test enemy's walkbox intersects with the player walkbox...
                let wb_test = enemy.get_walkbox_world();
                let player_test = game.player.get_walkbox_world();

                // Attempt at collision with attackbox
                if game.player.is_attacking {
                    let player_attack = game.player.get_attackbox_world();
                    if wb_test.has_intersection(player_attack) {
                        println!("Attack collided with enemy!");
                        enemy.damage(game.player.attack);
                        println!("damage done was {}", game.player.attack);

                        //Absorb Enemy
                        if enemy.power == true {
                            match enemy.kind {
                                EnemyKind::Health => {
                                    // Place gem on enemy's current tile
                                    // TODO: MAKE MORE ROBUST, CURRENTLY WON'T WORK ON NON GROUND TILES
                                    game.current_room_mut().tiles
                                        [((enemy.get_pos_y() - TOP_WALL) / 64) as usize]
                                        [((enemy.get_pos_x() - LEFT_WALL) / 64) as usize]
                                        .place_gem(Gem::Red);
                                    //game.player.plus_power_health();
                                    //println!("PowerupHealth is {}", game.player.power_up_vec[0]);
                                    //println!("Max Health is: {}", game.player.max_hp());
                                },
                                EnemyKind::Speed => {
                                    game.current_room_mut().tiles
                                        [((enemy.get_pos_y() - TOP_WALL) / 64) as usize]
                                        [((enemy.get_pos_x() - LEFT_WALL) / 64) as usize]
                                        .place_gem(Gem::Blue);
                                    //game.player.plus_power_speed();
                                    //println!("PowerupSpeed is {}", game.player.power_up_vec[1]);
                                },
                                EnemyKind::Attack => {
                                    game.current_room_mut().tiles
                                        [((enemy.get_pos_y() - TOP_WALL) / 64) as usize]
                                        [((enemy.get_pos_x() - LEFT_WALL) / 64) as usize]
                                        .place_gem(Gem::Yellow);
                                    //game.player.plus_power_attack();
                                    //println!("PowerupAttack is {}", game.player.power_up_vec[2]);
                                },
                            }
                            
                            enemy.power = false;
                        }
                    }
                }

                // Then there's a collision!
                if wb_test.has_intersection(player_test) {
                    //Damage enemy also! For some reason
                    //println!("Collision");
                    //enemy.damage(1);
                    // Check to see when the player was attacked last...
                    match game.player.last_invincibility_time {
                        // If there is an old invincibility time for the player,
                        // see if the "invincibility window" has elapsed since then...
                        Some( time ) => {
                            if time.elapsed() >= Duration::from_millis(1750) {
                                // If so, update the invincibility time and take damage to the player.
                                game.player.update_invincibility_time();
                                game.player.damage(1);
                            }
                        },
                        None => {
                            // Otherwise, take damage as there was
                            // no previous "invincibility window" to account for
                            game.player.update_invincibility_time();
                            game.player.damage(1);
                        }
                    }

                    // If the player is dead, update to the game over menu state
                    if game.player.death() {
                        *menu = MenuState::GameOver;
                    }
                }
            }
        }
            

        game.current_room_mut().enemies = enemy_list;

        core.wincan.set_draw_color(Color::RGBA(128, 0, 0, 255));
        let mut x = 0;
        let mut y = 0;
        // This can't be done with the current room function bc it returns a reference which messes up internal stuff
        for row in &game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tiles {
            for t in row {
                match t.walkability() {
                    Walkability::Wall | Walkability::Rock | Walkability::Pit => {
                        // Hacky af block collision that needs to be changed later
                        let opt = game.player.get_walkbox_world().intersection(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64));

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
    }