use crate::game::*;
use crate::util::*;
use crate::tile::*;
use crate::menu::*;
use crate::player::PowerUp;
use crate::entity::*;
use roguelike::SDLCore;

use std::time::Duration;
use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;



pub fn base(mut game : &mut Game, mut core : &mut SDLCore, mut menu : &mut MenuState, &debug: &bool) -> Result<(), String> {

// MOVE SOMEWHERE ELSE, TEXTURES SHOULD ONLY BE INITIALIZED ONCE
    let texture_creator = core.wincan.texture_creator();

    // Scope enums for readability
   	//use::MenuState::*;

    // Determine what to draw depending on state of the menu.
    match menu {

        MenuState::MainMenu => {
            let main_menu = texture_creator.load_texture("assets/main_menu.png")?;
            core.wincan.copy(&main_menu, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;
        }

        MenuState::GameActive => {
            // Load textures
            let bg = texture_creator.load_texture("assets/test_image.png")?;

            let slime_up = texture_creator.load_texture("assets/slime_up.png")?;
            let slime_down = texture_creator.load_texture("assets/slime_down.png")?;
            let slime_left = texture_creator.load_texture("assets/slime_left.png")?;
            let slime_right = texture_creator.load_texture("assets/slime_right.png")?;

            let speed_idle = texture_creator.load_texture("assets/speed_idle.png")?;
            let attack_idle = texture_creator.load_texture("assets/wizard_attack_enemy.png")?;
            let health_idle = texture_creator.load_texture("assets/health-sprite-down.png")?;

            let health_atk = texture_creator.load_texture("assets/health-projectile.png")?;
            let speed_atk = texture_creator.load_texture("assets/speed-projectile.png")?;
            let attack_atk = texture_creator.load_texture("assets/attack-projectile.png")?;

            let hp_indicator = texture_creator.load_texture("assets/hp.png")?;
            let hp_bomb_indicator = texture_creator.load_texture("assets/-3.png")?;

            //power assets
            let p_text = texture_creator.load_texture("assets/p_text.png")?;
            let p_text_health = texture_creator.load_texture("assets/p_text_health.png")?;
            let p_text_speed = texture_creator.load_texture("assets/p_text_speed.png")?;
            let p_text_attack = texture_creator.load_texture("assets/p_text_attack.png")?;
            //let p_empty = texture_creator.load_texture("assets/p_empty.png")?;
            let p_background = texture_creator.load_texture("assets/p_background.png")?;
            let p_blue_1 = texture_creator.load_texture("assets/p_blue_1.png")?;
            let p_blue_2 = texture_creator.load_texture("assets/p_blue_2.png")?;
            let p_blue_3 = texture_creator.load_texture("assets/p_blue_3.png")?;
            let p_red_1 = texture_creator.load_texture("assets/p_red_1.png")?;
            let p_red_2 = texture_creator.load_texture("assets/p_red_2.png")?;
            let p_red_3 = texture_creator.load_texture("assets/p_red_3.png")?;
            let p_yellow_1 = texture_creator.load_texture("assets/p_yellow_1.png")?;
            let p_yellow_2 = texture_creator.load_texture("assets/p_yellow_2.png")?;
            let p_yellow_3 = texture_creator.load_texture("assets/p_yellow_3.png")?;


            let gem_red = texture_creator.load_texture("assets/gem_red.png")?;
            let gem_yellow = texture_creator.load_texture("assets/gem_yellow.png")?;
            let gem_blue = texture_creator.load_texture("assets/gem_blue.png")?;
            let bomb_item = texture_creator.load_texture("assets/Bomb.png")?;
            let bomb_menu = texture_creator.load_texture("assets/bomb_menu.png")?;
            let bomb_explosion = texture_creator.load_texture("assets/Explosion.png")?;

            let bricks = texture_creator.load_texture("assets/ground_tile.png")?;
            let rock = texture_creator.load_texture("assets/rock.png")?;
            let spike = texture_creator.load_texture("assets/spike.png")?;

            let key = texture_creator.load_texture("assets/key.png")?;
            let td_locked = texture_creator.load_texture("assets/trapdoor_locked.png")?;

            let pl_heart = texture_creator.load_texture("assets/playerheart16x.png")?;

            // Draw black screen
            core.wincan.set_draw_color(Color::BLACK);
            core.wincan.clear();

            let mut yo = 0;
            let mut xo = 0;
            let mut x_dir = 0.0;
            let mut y_dir = 0.0;

            match game.game_state {
                GameState::Gameplay => {
                    // Draw background of game screen
                    core.wincan.copy(&bg, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;
                }
                GameState::InitialFloorTrans => {
                    // Draw background of game screen again, room transition is custom
                    core.wincan.copy(&bg, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;

                    // BETWEEN FLOORS DRAWING CODE IS AT BOTTOM BC IT NEEDS TO BE DRAWN OVERTOP
                }
                GameState::BetweenFloors => {
                    // Draw background of game screen again, room transition is custom
                    core.wincan.copy(&bg, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;

                    // BETWEEN FLOORS DRAWING CODE IS AT BOTTOM BC IT NEEDS TO BE DRAWN OVERTOP
                }
                GameState::BetweenRooms => {
                    // Transition duration
                    let dur = Duration::new(0, 400_000_000); // Half billion = half second
                    // 0.0 to 1.0 value to scale transition by
                    let scale = (game.transition_start.elapsed().as_millis() as f64 / dur.as_millis() as f64);

                    // Scales values to make proper directions
                    x_dir = match game.trans_dir {
                        Direction::Right => -1.0,
                        Direction::Left => 1.0,
                        _ => 0.0,
                    };

                    y_dir = match game.trans_dir {
                        Direction::Up => 1.0,
                        Direction::Down => -1.0,
                        _ => 0.0,
                    };

                    yo = (scale * y_dir * 720.0) as i32;
                    xo = (scale * x_dir * 1280.0) as i32;

                    // Draw backround tiles
                    core.wincan.copy(&bg, None, Rect::new(xo, yo, WINDOW_WIDTH, WINDOW_HEIGHT))?;
                    core.wincan.copy(&bg, None, Rect::new(xo + x_dir as i32 * -1280, yo + y_dir as i32 * -720, WINDOW_WIDTH, WINDOW_HEIGHT))?;

                    let x = 0;
                    let y = 0;
                    let rmx = game.cr.x + match game.trans_dir {
                        Direction::Right => -1,
                        Direction::Left => 1,
                        _ => 0,
                    };
                    let rmy = game.cr.y + match game.trans_dir {
                        Direction::Up => 1,
                        Direction::Down => -1,
                        _ => 0,
                    };

                    // THIS DRAWS THE PREVIOUS ROOM
                    let mut x = 0;
                    let mut y = 0;
                    for row in &game.map.floors[game.cf].rooms[rmy as usize][rmx as usize].tiles {
                        for t in row {
                            let x_val =
                                if xo != 0 {
                                    LEFT_WALL + x * 64 + xo + x_dir as i32
                                }
                                else {
                                    LEFT_WALL + x * 64
                                };

                            let y_val =
                                if yo != 0 {
                                    TOP_WALL + y * 64 + yo + y_dir as i32
                                }
                                else {
                                    TOP_WALL + y * 64
                                };

                            match t.sprite() {
                                SpriteID::Ground => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                }

                                // GEMS
                                SpriteID::GemRed => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&gem_red, None, Rect::new(x_val, y_val, 64, 64))?;
                                }
                                SpriteID::GemBlue => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&gem_blue, None, Rect::new(x_val, y_val, 64, 64))?;
                                }
                                SpriteID::GemYellow => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&gem_yellow, None, Rect::new(x_val, y_val, 64, 64))?;
                                }

                                // Do nothing, we already drew the surrounding walls as one image.
                                SpriteID::Wall => (),

                                SpriteID::Rock => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&rock, None, Rect::new(x_val, y_val, 64, 64))?;
                                    //core.wincan.copy(&gem_yellow, None, Rect::new(x_val, y_val, 64, 64))?;
                                }

                                SpriteID::Pit => {
                                    //core.wincan.set_draw_color(Color::RGBA(255, 255, 0, 255));
                                    //core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64));
                                }

                                SpriteID::DoorLocked => {
                                    core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                                    core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64))?;
                                }
                                SpriteID::DoorUnlocked => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    //core.wincan.set_draw_color(Color::RGBA(0, 255, 0, 255));
                                    //core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64));
                                }

                                SpriteID::Key => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&key, None, Rect::new(x_val, y_val, 64, 64))?;
                                }

                                SpriteID::Bomb => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&bomb_item, None, Rect::new(x_val, y_val, 64, 64))?;
                                }

                                SpriteID::Explosion => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&bomb_explosion, None, Rect::new(x_val, y_val, 64, 64))?;
                                }

                                SpriteID::TrapdoorLocked => {
                                    core.wincan.copy(&td_locked, None, Rect::new(x_val, y_val, 64, 64))?;
                                }

                                SpriteID::TrapdoorUnlocked => {
                                    core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                                    core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64))?;
                                }

                                SpriteID::Spike => {
                                    core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                                    core.wincan.copy(&spike, None, Rect::new(x_val, y_val, 64, 64))?;
                                    if t.has_gem() {
                                        match t.get_gem_type() {
                                            Gem::Red => {
                                                core.wincan.copy(&gem_red, None, Rect::new(x_val, y_val, 64, 64))?;
                                            }
                                            Gem::Blue => {
                                                core.wincan.copy(&gem_blue, None, Rect::new(x_val, y_val, 64, 64))?;
                                            }
                                            Gem::Yellow => {
                                                core.wincan.copy(&gem_yellow, None, Rect::new(x_val, y_val, 64, 64))?;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            x += 1;
                        }
                        y += 1;
                        x = 0;
                    }
                }
            }

            let mut x = 0;
            let mut y = 0;
            for row in &game.current_room().tiles {
                for t in row {
                    let x_val =
                    if xo != 0 {
                        LEFT_WALL + x * 64 + xo + x_dir as i32 * -1280
                    }
                    else {
                        LEFT_WALL + x * 64
                    };

                    let y_val =
                        if yo != 0 {
                            TOP_WALL + y * 64 + yo + y_dir as i32 * - 720
                        }
                        else {
                            TOP_WALL + y * 64
                        };

                    match t.sprite() {
                        SpriteID::Ground => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                        }

                        // GEMS
                        SpriteID::GemRed => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&gem_red, None, Rect::new(x_val, y_val, 64, 64))?;
                        }
                        SpriteID::GemBlue => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&gem_blue, None, Rect::new(x_val, y_val, 64, 64))?;
                        }
                        SpriteID::GemYellow => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&gem_yellow, None, Rect::new(x_val, y_val, 64, 64))?;
                        }

                        // Do nothing, we already drew the surrounding walls as one image.
                        SpriteID::Wall => (),

                        SpriteID::Rock => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&rock, None, Rect::new(x_val, y_val, 64, 64))?;
                            //core.wincan.copy(&gem_yellow, None, Rect::new(x_val, y_val, 64, 64))?;
                        }

                        SpriteID::Pit => {
                            //core.wincan.set_draw_color(Color::RGBA(255, 255, 0, 255));
                            //core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64));
                        }

                        SpriteID::Bomb => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&bomb_item, None, Rect::new(x_val, y_val, 64, 64))?;
                        }

                        SpriteID::Explosion => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&bomb_explosion, None, Rect::new(x_val, y_val, 64, 64))?;
                        }

                        SpriteID::DoorLocked => {
                            core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                            core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64))?;
                        }
                        SpriteID::DoorUnlocked => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            //core.wincan.set_draw_color(Color::RGBA(0, 255, 0, 255));
                            //core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64));
                        }

                        SpriteID::Key => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&key, None, Rect::new(x_val, y_val, 64, 64))?;
                        }

                        SpriteID::TrapdoorLocked => {
                            core.wincan.copy(&td_locked, None, Rect::new(x_val, y_val, 64, 64))?;
                        }

                        SpriteID::TrapdoorUnlocked => {
                            core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                            core.wincan.draw_rect(Rect::new(x_val, y_val, 64, 64))?;
                        }

                        SpriteID::Spike => {
                            core.wincan.copy(&bricks, None, Rect::new(x_val, y_val, 64, 64))?;
                            core.wincan.copy(&spike, None, Rect::new(x_val, y_val, 64, 64))?;
                            if t.has_gem() {
                                match t.get_gem_type() {
                                    Gem::Red => {
                                        core.wincan.copy(&gem_red, None, Rect::new(x_val, y_val, 64, 64))?;
                                    }
                                    Gem::Blue => {
                                        core.wincan.copy(&gem_blue, None, Rect::new(x_val, y_val, 64, 64))?;
                                    }
                                    Gem::Yellow => {
                                        core.wincan.copy(&gem_yellow, None, Rect::new(x_val, y_val, 64, 64))?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    x += 1;
                }
                y += 1;
                x = 0;
            }

            let x_val =
                if xo != 0 {
                    xo + x_dir as i32 * -1280
                }
                else {
                    0
                };

            let y_val =
                if yo != 0 {
                    yo + y_dir as i32 * - 720
                }
                else {
                    0
                };
            match game.player.get_dir() {
                Direction::Up => {
                    core.wincan.copy(&slime_up, None,
                        Rect::new(
                            game.player.get_pos_x() - 35 + 4 + x_val,
                            game.player.get_pos_y() - 64 + (game.player.box_es.get_walkbox(game.player.pos).height()/2) as i32 + y_val,
                            64, 64)
                        )?;
                }
                Direction::Down => {
                    core.wincan.copy(&slime_down, None,
                        Rect::new(
                            game.player.get_pos_x() - 35 + x_val,
                            game.player.get_pos_y() - 64 + (game.player.box_es.get_walkbox(game.player.pos).height()/2) as i32 + y_val,
                            64, 64)
                        )?;
                }
                Direction::Left => {
                    core.wincan.copy(&slime_left, None,
                        Rect::new(
                            game.player.get_pos_x() - 35 + 4 + x_val,
                            game.player.get_pos_y() - 64 + (game.player.box_es.get_walkbox(game.player.pos).height()/2) as i32 + y_val,
                            64, 64)
                        )?;
                }
                Direction::Right => {
                    core.wincan.copy(&slime_right, None,
                        Rect::new(
                            game.player.get_pos_x() - 35 + x_val,
                            game.player.get_pos_y() - 64 + (game.player.box_es.get_walkbox(game.player.pos).height()/2) as i32 + y_val,
                            64, 64)
                        )?;
                }
            }

            //draw_enemies(textures);

            let enemies = &mut game.current_room_mut().enemies;
            for enemy in enemies.iter_mut()  {
                    let tex_a = match &enemy.kind {
                        EnemyKind::Attack => &attack_atk,
                        EnemyKind::Health => &health_atk,
                        EnemyKind::Speed => &speed_atk
                    };

                for atk in &enemy.atk_list {
                    core.wincan.copy(&tex_a, None,
                        Rect::new(
                                atk.pos.x as i32 - (atk.box_es.hitbox.x/2) as i32,
                                atk.pos.y as i32 - (atk.box_es.hitbox.y) as i32,
                                atk.box_es.hitbox.x,
                                atk.box_es.hitbox.y)
                    )?;
                }
                if !enemy.death() {
                    let tex = match &enemy.kind {
                        EnemyKind::Attack => &attack_idle,
                        EnemyKind::Health => &health_idle,
                        EnemyKind::Speed => &speed_idle
                    };

                    core.wincan.copy(&tex, None,
                        Rect::new(
                            enemy.get_pos_x() - 35 + 4 + x_val,
                            enemy.get_pos_y() - 64 + (enemy.box_es.get_walkbox(enemy.pos).height()/2) as i32 + y_val,
                            64, 64)
                    )?;
                    
                    core.wincan.set_draw_color(Color::RGBA(139, 195, 74, 255));
                    if enemy.recently_attacked() {
                        core.wincan.fill_rect(enemy.box_es.get_attackbox(enemy.pos, enemy.dir))?;
                    }
                }
            }

            // If the player was attacked, show a quick damage indicator ("-1" in red)
            if game.player.was_attacked() {
                core.wincan.copy(&hp_indicator, None, Rect::new(game.player.get_pos_x() as i32, game.player.get_pos_y() as i32, 64, 64))?;
            }

            let mut flip_heart = false;
            let mut hp_offset_x = 0;
            let mut hp_offset_y = 0;
            for i in 0 .. game.player.hp {
                if i > 0 {
                    if i % 2 == 0 {
                        hp_offset_x += 1;
                    }

                    if i % 6 == 0 {
                        hp_offset_x = 0;
                        hp_offset_y += 34;
                    }
                }
                core.wincan.copy_ex(&pl_heart, None, Rect::new(10 + ( i % 6 ) * 28 + hp_offset_x, 40 + hp_offset_y, 28, 48), 0.0, None, flip_heart, false)?;
                flip_heart = !flip_heart;
            }

            //draw powerup dials
            core.wincan.copy(&p_text, None, Rect::new(80,468,64,64))?;
            core.wincan.copy(&p_text_health, None, Rect::new(0,532,64,64))?;
            core.wincan.copy(&p_text_speed, None, Rect::new(0, 596,64,64))?;
            core.wincan.copy(&p_text_attack, None, Rect::new(0,660,64,64))?;
            core.wincan.copy(&p_background, None, Rect::new(80,532,64,64))?;
            core.wincan.copy(&p_background, None, Rect::new(80,596,64,64))?;
            core.wincan.copy(&p_background, None, Rect::new(80,660,64,64))?;

            // TODO: REWORK INTO MATCH STATEMENT BECAUSE THIS IS A MESS
            if game.player.power_image_health() == 1 {
                core.wincan.copy(&p_red_1, None, Rect::new(80,532,64,64))?;
            }
            else if game.player.power_image_health() == 2 {
                core.wincan.copy(&p_red_2, None, Rect::new(80,532,64,64))?;
            }
            else if game.player.power_image_health() == 3 {
                core.wincan.copy(&p_red_3, None, Rect::new(80,532,64,64))?;
            }
            if game.player.power_image_speed() == 1 {
                core.wincan.copy(&p_blue_1, None, Rect::new(80,596,64,64))?;
            }
            else if game.player.power_image_speed() == 2 {
                core.wincan.copy(&p_blue_2, None, Rect::new(80,596,64,64))?;
            }
            else if game.player.power_image_speed() == 3 {
                core.wincan.copy(&p_blue_3, None, Rect::new(80,596,64,64))?;
            }
            if game.player.power_image_attack() == 1 {
                core.wincan.copy(&p_yellow_1, None, Rect::new(80,660,64,64))?;
            }
            else if game.player.power_image_attack() == 2 {
                core.wincan.copy(&p_yellow_2, None, Rect::new(80,660,64,64))?;
            }
            else if game.player.power_image_attack() == 3 {
                core.wincan.copy(&p_yellow_3, None, Rect::new(80,660,64,64))?;
            }

            // ------------------------ DRAW UI --------------------------

            // Rough key setup
            if game.player.has_key {
                core.wincan.copy(&key, None, Rect::new(96, 200, 64, 64))?;
            }

            if game.player.has_bomb {
                core.wincan.copy(&bomb_menu, None, Rect::new(16, 200, 64, 64))?;
            }

            // Minimap (commented out first block as the block below does the same thing)
            // for x in 0..8 {
            //     for y in 0..8 {
            //         // Current room
            //         if x == game.cr.x && y == game.cr.y {
            //             core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
            //             core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
            //         }
            //         // Visited rooms
            //         else if game.map.floors[game.cf].rooms[y as usize][x as usize].visited == true {
            //             core.wincan.set_draw_color(Color::RGBA(80, 80, 80, 255));
            //             core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
            //         }
            //         // Unvisited rooms
            //         else if game.map.floors[game.cf].rooms[y as usize][x as usize].visited == false &&
            //             game.map.floors[game.cf].rooms[y as usize][x as usize].exists == true {
            //             core.wincan.set_draw_color(Color::RGBA(30, 30, 30, 255));
            //             core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;

            //         }
            //         // Black border for separation
            //         core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
            //         core.wincan.draw_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
            //     }
            // }

            // Minimap
            for x in 0..8 {
                for y in 0..8 {
                    // Current room
                    if x == game.cr.x && y == game.cr.y {
                        core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
                        core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                    }
                    // Visited rooms
                    else if game.map.floors[game.cf].rooms[y as usize][x as usize].visited == true {
                        core.wincan.set_draw_color(Color::RGBA(80, 80, 80, 255));
                        core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                    }
                    // Unvisited rooms
                    else if game.map.floors[game.cf].rooms[y as usize][x as usize].visited == false &&
                        game.map.floors[game.cf].rooms[y as usize][x as usize].exists == true {
                        core.wincan.set_draw_color(Color::RGBA(30, 30, 30, 255));
                        core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;

                    }
                    // Black border for separation
                    core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                    core.wincan.draw_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                }
            }

            if debug {
                // Draw player collision hitbox
                core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                // removing previous hitbox debug for boxes.rs
                //core.wincan.draw_rect(game.player.get_walkbox_world())?;

                let enemies = &mut game.current_room_mut().enemies;

                for enemy in enemies.iter_mut() {
                    if !enemy.death() {
                        core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                        core.wincan.draw_rect(enemy.box_es.get_walkbox(enemy.pos))?;

                        core.wincan.set_draw_color(Color::RGBA(128,128,255,255));
                        core.wincan.draw_rect(
                            Rect::new(
                                enemy.get_pos_x() - (enemy.box_es.hitbox.x/2) as i32,
                                enemy.get_pos_y() - (enemy.box_es.hitbox.y) as i32,
                                enemy.box_es.hitbox.x,
                                enemy.box_es.hitbox.y
                            )
                        )?;
                        
                    }

                    for atk in &enemy.atk_list {
                        core.wincan.set_draw_color(Color::RGBA(128,128,255,255));
                        core.wincan.draw_rect(
                            Rect::new(
                                atk.pos.x as i32 - (atk.box_es.hitbox.x/2) as i32,
                                atk.pos.y as i32 - (atk.box_es.hitbox.y) as i32,
                                atk.box_es.hitbox.x,
                                atk.box_es.hitbox.y
                            )
                        )?;
                    }
                }

                // Draw player damage hitbox
                //core.wincan.set_draw_color(Color::RGBA(128, 128, 255, 255));
                // core.wincan.draw_rect(Rect::new(game.player.get_pos_x() - (game.player.get_hitbox_x()/2) as i32,
                //                                     game.player.get_pos_y() - (game.player.get_hitbox_y()) as i32 + (game.player.get_walkbox().height()/2) as i32,
                //                                     game.player.get_hitbox_x(),
                //                                     game.player.get_hitbox_y())
                //                             )?;
                // Draw debug of walkbox from boxes.rs for testing
                core.wincan.set_draw_color(Color::RGBA(128, 0, 128, 255));
                core.wincan.draw_rect(game.player.box_es.get_walkbox(game.player.pos)
                                            )?;

                // Draw debug of hitbox from boxes.rs for testing
                core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
                core.wincan.draw_rect(game.player.box_es.get_hitbox(game.player.pos)
                                            )?;

                // Draw null at center of player hitbox
                core.wincan.set_draw_color(Color::RGBA(255, 0, 255, 255));
                core.wincan.draw_line(
                    Point::new(game.player.get_pos_x() + 4, game.player.get_pos_y()),
                    Point::new(game.player.get_pos_x() - 4, game.player.get_pos_y()),
                )?;
                core.wincan.draw_line(
                    Point::new(game.player.get_pos_x(), game.player.get_pos_y() + 4),
                    Point::new(game.player.get_pos_x(), game.player.get_pos_y() - 4),
                )?;

                // Draw collision hitboxes
                core.wincan.set_draw_color(Color::RGBA(128, 0, 0, 255));
                x = 0;
                y = 0;
                for row in &game.current_room().tiles {
                    for t in row {
                        match t.walkability() {

                            Walkability::Wall | Walkability::Rock | Walkability::Pit => {
                                core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }

                            _ => (),
                            // Dont draw anything for other tiles
                        }
                        x += 1;
                    }
                    y += 1;
                    x = 0;
                }

                // Draw a box over the current tile
                core.wincan.set_draw_color(Color::RGBA(255, 255, 0, 255));
                if game.player.current_frame_tile != game.player.prev_frame_tile {
                    core.wincan.fill_rect(Rect::new((game.player.get_pos_x() - LEFT_WALL) / 64 * 64 + LEFT_WALL,
                                                        (game.player.get_pos_y() - TOP_WALL) / 64 * 64 + TOP_WALL,
                                                        64,
                                                        65,
                    ))?;

                }
                else {
                    core.wincan.draw_rect(Rect::new((game.player.get_pos_x() - LEFT_WALL) / 64 * 64 + LEFT_WALL,
                                                        (game.player.get_pos_y() - TOP_WALL) / 64 * 64 + TOP_WALL,
                                                        64,
                                                        65,
                    ))?;
                }
            }

            // Draw attackbox
            core.wincan.set_draw_color(Color::RGBA(139, 195, 74, 255));
            if game.player.recently_attacked() {
                //core.wincan.fill_rect(game.player.get_attackbox_world())?;  //removed for boxes.es
                core.wincan.fill_rect(game.player.box_es.get_attackbox(game.player.pos, game.player.dir))?;
            }

            if game.player.recently_bombed() {
                //core.wincan.fill_rect(game.player.get_attackbox_world())?;  //removed for boxes.es
                core.wincan.copy(&bomb_explosion, None, game.player.box_es.get_bombbox(game.player.pos_static, game.player.dir))?;
                //core.wincan.fill_rect(game.player.box_es.get_bombbox(game.player.pos, game.player.dir))?;
            }
            // FINAL DRAW FOR ANY OVERLAYS
            match game.game_state {
                GameState::InitialFloorTrans => {
                    let dur = Duration::new(0, 500_000_000); // 1 billion = second
                    let ms = game.transition_start.elapsed().as_millis();

                    if ms <= 1500 {
                        core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                        core.wincan.fill_rect(Rect::new(0, 0, 1280, 720))?;

                        let f1 = texture_creator.load_texture("assets/floor_1.png")?;
                        core.wincan.copy(&f1, None, Rect::new(420, 290, 64 * 8, 15 * 8))?;
                    } else if ms <= 2000 {
                        let scale = 1.0 - ((game.transition_start.elapsed().as_millis() - 1500) as f64 / dur.as_millis() as f64);

                        core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                        core.wincan.fill_rect(Rect::new(0, 0, 1280, (scale * 360.0) as u32))?;
                        core.wincan.fill_rect(Rect::new(0, (720.0 - 360.0 * scale) as i32, 1280, (scale * 360.0) as u32))?;
                    }
                }
                GameState::BetweenFloors => {


                    // Transition duration
                    let dur = Duration::new(0, 500_000_000); // 1 billion = second

                    let ms = game.transition_start.elapsed().as_millis();
                    if ms <= 500 {
                        // 0.0 to 1.0 value to scale transition by
                        let scale = (game.transition_start.elapsed().as_millis() as f64 / dur.as_millis() as f64);

                        core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                        core.wincan.fill_rect(Rect::new(0, 0, 1280, (scale * 360.0) as u32))?;
                        core.wincan.fill_rect(Rect::new(0, (720.0 - 360.0 * scale) as i32, 1280, (scale * 360.0) as u32))?;
                    } else if ms <= 2500 {
                        core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                        core.wincan.fill_rect(Rect::new(0, 0, 1280, 720))?;

                        if ms > 550 { // Avoids drawing previous floor for a few frames bug
                            match game.cf {
                                0 => {
                                    let f1 = texture_creator.load_texture("assets/floor_1.png")?;
                                    core.wincan.copy(&f1, None, Rect::new(420, 290, 64 * 8, 15 * 8))?;
                                }
                                1 => {
                                    let f2 = texture_creator.load_texture("assets/floor_2.png")?;
                                    core.wincan.copy(&f2, None, Rect::new(420, 290, 64 * 8, 15 * 8))?;
                                }
                                2 => {
                                    let f3 = texture_creator.load_texture("assets/floor_3.png")?;
                                    core.wincan.copy(&f3, None, Rect::new(420, 290, 64 * 8, 15 * 8))?;
                                }
                                _ => {}
                            }
                        }
                    } else if ms <= 3000 {
                        let scale = 1.0 - ((game.transition_start.elapsed().as_millis() - 2500) as f64 / dur.as_millis() as f64);

                        core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                        core.wincan.fill_rect(Rect::new(0, 0, 1280, (scale * 360.0) as u32))?;
                        core.wincan.fill_rect(Rect::new(0, (720.0 - 360.0 * scale) as i32, 1280, (scale * 360.0) as u32))?;
                    }
                }
                _ => {} // Do nothing for other stuff
            }
        }

        MenuState::GameOver => {
            let gameover = texture_creator.load_texture("assets/game_over.png")?;
            core.wincan.copy(&gameover, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;
        }

        MenuState::GamePaused => {
            core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
            core.wincan.clear();

            let pause_menu = texture_creator.load_texture("assets/pause_menu.png")?;
            core.wincan.copy(&pause_menu, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;
        }

    }




    // Tell SDL to draw everything on screen.
    core.wincan.present();

    Ok(())
}
