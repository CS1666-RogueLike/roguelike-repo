use crate::game::*;
use crate::util::*;
use crate::tile::*;
use crate::menu::*;
use crate::player::PowerUp;
use crate::entity::*;
use roguelike::SDLCore;

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

            let hp_indicator = texture_creator.load_texture("assets/hp.png")?;

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

            let bricks = texture_creator.load_texture("assets/ground_tile.png")?;
            let rock = texture_creator.load_texture("assets/rock.png")?;
            let spike = texture_creator.load_texture("assets/spike.png")?;

            let key = texture_creator.load_texture("assets/key.png")?;
            let td_locked = texture_creator.load_texture("assets/trapdoor_locked.png")?;

            let pl_heart = texture_creator.load_texture("assets/playerheart16x16.png")?;

            // Draw black screen
            core.wincan.set_draw_color(Color::BLACK);
            core.wincan.clear();

            // Draw background of game screen
            core.wincan.copy(&bg, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;

            let mut x = 0;
            let mut y = 0;
            for row in &game.current_room().tiles {
                for t in row {
                    match t.sprite() {
                        SpriteID::Ground => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }

                        // GEMS
                        SpriteID::GemRed => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            core.wincan.copy(&gem_red, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }
                        SpriteID::GemBlue => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            core.wincan.copy(&gem_blue, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }
                        SpriteID::GemYellow => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            core.wincan.copy(&gem_yellow, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }

                        // Do nothing, we already drew the surrounding walls as one image.
                        SpriteID::Wall => (),

                        SpriteID::Rock => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            core.wincan.copy(&rock, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            //core.wincan.copy(&gem_yellow, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }

                        SpriteID::Pit => {
                            //core.wincan.set_draw_color(Color::RGBA(255, 255, 0, 255));
                            //core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64));
                        }

                        SpriteID::DoorLocked => {
                            core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                            core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }
                        SpriteID::DoorUnlocked => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            //core.wincan.set_draw_color(Color::RGBA(0, 255, 0, 255));
                            //core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64));
                        }

                        SpriteID::Key => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            core.wincan.copy(&key, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }

                        SpriteID::TrapdoorLocked => {
                            core.wincan.copy(&td_locked, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }

                        SpriteID::TrapdoorUnlocked => {
                            core.wincan.set_draw_color(Color::RGBA(255, 128, 128, 255));
                            core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }

                        SpriteID::Spike => {
                            core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            core.wincan.copy(&spike, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                        }
                    }
                    x += 1;
                }
                y += 1;
                x = 0;
            }

            match game.player.get_dir() {
                Direction::Up => {
                    core.wincan.copy(&slime_up, None,
                        Rect::new(
                            game.player.get_pos_x() - 35 + 4,
                            game.player.get_pos_y() - 64 + (game.player.get_walkbox().height()/2) as i32,
                            64, 64)
                        )?;
                }
                Direction::Down => {
                    core.wincan.copy(&slime_down, None,
                        Rect::new(
                            game.player.get_pos_x() - 35,
                            game.player.get_pos_y() - 64 + (game.player.get_walkbox().height()/2) as i32,
                            64, 64)
                        )?;
                }
                Direction::Left => {
                    core.wincan.copy(&slime_left, None,
                        Rect::new(
                            game.player.get_pos_x() - 35 + 4,
                            game.player.get_pos_y() - 64 + (game.player.get_walkbox().height()/2) as i32,
                            64, 64)
                        )?;
                }
                Direction::Right => {
                    core.wincan.copy(&slime_right, None,
                        Rect::new(
                            game.player.get_pos_x() - 35,
                            game.player.get_pos_y() - 64 + (game.player.get_walkbox().height()/2) as i32,
                            64, 64)
                        )?;
                }
            }

            //draw_enemies(textures);

            let enemies = &mut game.current_room_mut().enemies;
            for enemy in enemies.iter_mut()  {
                if !enemy.death() {
                    let tex = match &enemy.kind {
                        EnemyKind::Attack => &attack_idle,
                        EnemyKind::Health => &health_idle,
                        EnemyKind::Speed => &speed_idle
                    };
    
                    core.wincan.copy(&tex, None,
                        Rect::new(
                            enemy.get_pos_x() - 35 + 4,
                            enemy.get_pos_y() - 64 + (enemy.get_walkbox().height()/2) as i32,
                            64, 64)
                    )?;
                }
            }

            // If the player was attacked, show a quick damage indicator ("-1" in red)
            if game.player.was_attacked() {
                core.wincan.copy(&hp_indicator, None, Rect::new(game.player.get_pos_x() as i32, game.player.get_pos_y() as i32, 64, 64))?;
            }

            for i in 0 .. game.player.hp {
                core.wincan.copy(&pl_heart, None, Rect::new(1 + (i * 63), 40, 64, 64))?;
            }

            //draw powerup dials
            core.wincan.copy(&p_text, None, Rect::new(80,468,64,64))?;
            core.wincan.copy(&p_text_health, None, Rect::new(0,532,64,64))?;
            core.wincan.copy(&p_text_speed, None, Rect::new(0, 596,64,64))?;
            core.wincan.copy(&p_text_attack, None, Rect::new(0,660,64,64))?;
            core.wincan.copy(&p_background, None, Rect::new(80,532,64,64))?;
            core.wincan.copy(&p_background, None, Rect::new(80,596,64,64))?;
            core.wincan.copy(&p_background, None, Rect::new(80,660,64,64))?;

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
                core.wincan.copy(&key, None, Rect::new(64, 200, 64, 64))?;
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
                core.wincan.draw_rect(game.player.get_walkbox_world())?;

                let enemies = &mut game.current_room_mut().enemies;
              
                for enemy in enemies.iter_mut() {
                    if !enemy.death() {
                        core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                        core.wincan.draw_rect(enemy.get_walkbox_world())?;
    
                        core.wincan.set_draw_color(Color::RGBA(128,128,255,255));
                        core.wincan.draw_rect(
                            Rect::new(
                                enemy.get_pos_x() - (enemy.get_hitbox_x()/2) as i32,
                                enemy.get_pos_y() - (enemy.get_hitbox_y()) as i32,
                                enemy.get_hitbox_x(),
                                enemy.get_hitbox_y()
                            )
                        )?;
                    }
                }

                // Draw player damage hitbox
                core.wincan.set_draw_color(Color::RGBA(128, 128, 255, 255));
                core.wincan.draw_rect(Rect::new(game.player.get_pos_x() - (game.player.get_hitbox_x()/2) as i32,
                                                    game.player.get_pos_y() - (game.player.get_hitbox_y()) as i32 + (game.player.get_walkbox().height()/2) as i32,
                                                    game.player.get_hitbox_x(),
                                                    game.player.get_hitbox_y())
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
                core.wincan.fill_rect(game.player.get_attackbox_world())?;
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

