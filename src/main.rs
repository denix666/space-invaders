#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};

mod resources;
use resources::Resources;

mod player;
use player::Player;

mod game;
use game::Game;

mod enemy;
use enemy::Enemy;

mod block;
use block::Block;

mod bullet;
use bullet::Bullet;

mod bomb;
use bomb::Bomb;

mod ufo;
use ufo::Ufo;

fn window_conf() -> Conf {
    let mut title = String::from("Space Invaders v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: resources::WINDOW_WIDTH,
        window_height: resources::WINDOW_HEIGHT,
        ..Default::default()
    }
}

pub enum GameState {
    Intro,
    InitLevel,
    Game,
    LevelFail,
    Paused,
    GameOver,
    LevelCompleted,
}

async fn make_blocks_array(resources: &Resources) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    for y in 40..=45 {
        for x in 10..=20 {
            blocks.push(
                Block::new((x * 10) as f32, (y * 10) as f32, resources).await,
            );
        }

        for x in 30..=40 {
            blocks.push(
                Block::new((x * 10) as f32, (y * 10) as f32, resources).await,
            );
        }

        for x in 50..=60 {
            blocks.push(
                Block::new((x * 10) as f32, (y * 10) as f32, resources).await,
            );
        }
    }

    return blocks
}

async fn make_enemies_array() -> Vec<Enemy> {
    let mut enemies: Vec<Enemy> = Vec::new();

    let mut x: f32;
    for i in (80..=600).step_by(70) {
        x = i as f32;
        enemies.push(
            Enemy::new(x, 80.0, "e").await,
        );
    }

    for i in (80..=600).step_by(70) {
        x = i as f32;
        enemies.push(
            Enemy::new(x, 130.0, "a").await,
        );
    }

    for i in (80..=600).step_by(70) {
        x = i as f32;
        enemies.push(
            Enemy::new(x, 180.0, "b").await,
        );
    }

    return enemies
}

fn draw_info(font: Font, score: &str, hi_score: &str, lives: &str) {
    draw_line(0.0, 525.0, 700.0, 525.0, 1.0, BROWN);
    
    draw_text_ex("SCORE: ", 30.0, 545.0, 
        TextParams {
            font,
            font_size: 25,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(score, 155.0, 545.0, 
        TextParams {
            font,
            font_size: 25,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex("LIVES: ", 260.0, 545.0, 
        TextParams {
            font,
            font_size: 25,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(lives, 375.0, 545.0, 
        TextParams {
            font,
            font_size: 25,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex("HI-SCORE: ", 450.0, 545.0, 
        TextParams {
            font,
            font_size: 25,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(hi_score, 620.0, 545.0, 
        TextParams {
            font,
            font_size: 25,
            color: ORANGE,
            ..Default::default()
        },
    );
}

fn show_text(font: Font, header_text: &str, message_text: &str) {
    let header_dims = measure_text(header_text, Some(font), 50, 1.0);
    let message_dims = measure_text(message_text, Some(font), 20, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 50,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        280.0,
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub enum Dir {
    Left,
    Right,
}

#[macroquad::main(window_conf)]
async fn main() {
    let resources = Resources::new().await;
    let mut game_state = GameState::Intro;
    let mut player = Player::new(&resources).await;
    let mut enemies: Vec<Enemy> = make_enemies_array().await;
    let mut blocks: Vec<Block> = make_blocks_array(&resources).await;
    let mut game = Game::new().await;
    let mut enemy_direction: Dir = Dir::Left;
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut ufo: Vec<Ufo> = Vec::new();
    let mut bombs: Vec<Bomb> = Vec::new();
    let mut bomb_last_time: f64 = get_time();
    let mut time_between_bombs: f64;
    let mut ufo_last_time: f64 = get_time();
    let mut next_bonus_at: i32 = 1000;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                game.score = 0;
                game.lives = 3;
                game.mission = 1;
                game.enemy_speed = resources::ENEMY_INIT_SPEED;

                draw_texture(resources.intro, 0.0, 0.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::InitLevel;
                }
            },
            GameState::InitLevel => {
                bomb_last_time = get_time();
                player.x = 320.0;
                draw_info(resources.font, 
                    game.score.to_string().as_str(), 
                    game.hi_score.to_string().as_str(),
                  game.lives.to_string().as_str());

                for enemy in &mut enemies {
                    enemy.draw();
                }
                for block in &mut blocks {
                    block.draw();
                }
                
                let mut header_text = String::from("MISSION - ");
                header_text.push_str(&game.mission.to_string());
                show_text(resources.font, header_text.as_str(), "press 'space' to start...");

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            },
            GameState::Game => {
                draw_info(resources.font, 
                          game.score.to_string().as_str(), 
                          game.hi_score.to_string().as_str(),
                        game.lives.to_string().as_str());
                
                for bomb in &mut bombs {
                    bomb.update(get_frame_time());
                    bomb.draw();

                    if let Some(_i) = bomb.rect.intersect(player.rect) {
                        bomb.destroyed = true;
                        if game.lives > 0 {
                            game_state = GameState::LevelFail;
                        } else {
                            game_state = GameState::GameOver;
                        }
                    }

                    if bullets.len() > 0 {
                        if let Some(_i) = bomb.rect.intersect(bullets[0].rect) {
                            bomb.destroyed = true;
                            bullets[0].destroyed = true;
                            game.score += 5;
                        }
                    }
                }

                if get_time() - ufo_last_time > resources::MINIMAL_TIME_BETWEEN_EACH_UFO {
                    let from_side = match rand::thread_rng().gen_range(0..=1) {
                        0 => "left",
                        _ => "right",
                    };
                    ufo.push(
                        Ufo::new(from_side).await,
                    );
                    ufo_last_time = get_time();
                }

                if ufo.len() > 0 {
                    ufo[0].draw();
                    if bullets.len() > 0 {
                        if let Some(_i) = bullets[0].rect.intersect(ufo[0].rect) {
                            ufo[0].destroyed = true;
                            bullets[0].destroyed = true;
                            game.score += 100;
                        }
                    }
                }

                // generate random time between the bombs
                if get_time() - bomb_last_time > resources::MINIMAL_TIME_BETWEEN_BOMBS {
                    time_between_bombs = rand::thread_rng().gen_range(0.0..=40.0);

                    if enemies.len() > 0 {
                        if get_time() - bomb_last_time > resources::MINIMAL_TIME_BETWEEN_BOMBS + time_between_bombs {
                            let bomb_type = match rand::thread_rng().gen_range(0..=1) {
                                0 => "a",
                                _ => "b",
                            };
                            let enemy_index = rand::thread_rng().gen_range(0..enemies.len());
                            bombs.push(
                                Bomb::new(enemies[enemy_index].x + 25.0, enemies[enemy_index].y + 36.0, bomb_type.to_string().as_str()).await,
                            );
                            bomb_last_time = get_time();
                        }
                    }
                }
                
                if is_key_down(KeyCode::Up) {
                    if bullets.len() == 0 {
                        bullets.push(
                            Bullet::new(player.x + 32.0, player.y, &resources).await,
                        );
                    }
                }
                player.draw();

                if bullets.len() > 0 {
                    bullets[0].update(get_frame_time());
                    bullets[0].draw();
                }

                let mut need_to_pull_down: bool = false;
                for enemy in &mut enemies {
                    match enemy_direction {
                        Dir::Left => {
                            enemy.x -= 1.0 * game.enemy_speed;
                            if enemy.x < 0.0 {
                                need_to_pull_down = true;
                                enemy_direction = Dir::Right;
                            }
                        },
                        Dir::Right => {
                            enemy.x += 1.0 * game.enemy_speed;
                            if enemy.x > 650.0 {
                                need_to_pull_down = true;
                                enemy_direction = Dir::Left;
                            }
                        },
                    }
                    
                    enemy.draw();

                    if enemy.y + 30.0 > player.y {
                        game_state = GameState::GameOver;
                    } 

                    if bullets.len() > 0 {
                        if let Some(_i) = bullets[0].rect.intersect(enemy.rect) {
                            enemy.destroyed = true;
                            bullets[0].destroyed = true;
                            game.score += 10;
                        }
                    }

                    if let Some(_i) = player.rect.intersect(enemy.rect) {
                        game_state = GameState::GameOver;
                    }
                }

                for block in &mut blocks {
                    for enemy in &mut enemies {
                        if let Some(_i) = enemy.rect.intersect(block.rect) {
                            block.destroyed = true;
                        }
                    }
                    if bullets.len() > 0 {
                        if let Some(_i) = bullets[0].rect.intersect(block.rect) {
                            block.destroyed = true;
                            bullets[0].destroyed = true;
                        }
                    }
                    for bomb in &mut bombs {
                        if let Some(_i) = bomb.rect.intersect(block.rect) {
                            bomb.destroyed = true;
                            block.destroyed = true;
                        }
                    }
                }

                if need_to_pull_down {
                    for enemy in &mut enemies {
                        enemy.y += 10.0;
                    }
                    game.enemy_speed += 0.2;
                }

                for block in &mut blocks {
                    block.draw();
                }

                if game.score > game.hi_score {
                    game.hi_score = game.score;
                }

                if game.score > next_bonus_at {
                    game.lives += 1;
                    next_bonus_at += 1000;
                }

                if enemies.len() == 0 {
                    game_state = GameState::LevelCompleted;
                }
            },
            GameState::LevelFail => {
                draw_info(resources.font, 
                    game.score.to_string().as_str(), 
                    game.hi_score.to_string().as_str(),
                  game.lives.to_string().as_str());

                for enemy in &mut enemies {
                    enemy.draw();
                }
                for block in &mut blocks {
                    block.draw();
                }
                show_text(resources.font, "MISSION FAIL", "press 'space' to continue...");
                if is_key_pressed(KeyCode::Space) {
                    game.lives -= 1;
                    player.x = 320.0;
                    bullets.clear();
                    bombs.clear();
                    ufo.clear();
                    game_state = GameState::Game;
                }
            },
            GameState::Paused => {

            },
            GameState::LevelCompleted => {
                draw_info(resources.font, 
                    game.score.to_string().as_str(), 
                    game.hi_score.to_string().as_str(),
                  game.lives.to_string().as_str());
                for block in &mut blocks {
                    block.draw();
                }
                show_text(resources.font, "MISSION COMPLETED", "press 'space' to continue...");
                if is_key_pressed(KeyCode::Space) {
                    player.x = 320.0;
                    bullets.clear();
                    bombs.clear();
                    enemies.clear();
                    ufo.clear();
                    enemies = make_enemies_array().await;
                    blocks.clear();
                    blocks = make_blocks_array(&resources).await;
                    game.mission += 1;
                    game.enemy_speed = resources::ENEMY_INIT_SPEED + game.mission as f32 * 0.2;
                    game_state = GameState::InitLevel;
                }
            },
            GameState::GameOver => {
                draw_info(resources.font, 
                    game.score.to_string().as_str(), 
                    game.hi_score.to_string().as_str(),
                  game.lives.to_string().as_str());

                for enemy in &mut enemies {
                    enemy.draw();
                }
                for block in &mut blocks {
                    block.draw();
                }
                show_text(resources.font, "GAME OVER", "press 'space' to start new game...");
                if is_key_pressed(KeyCode::Space) {
                    bullets.clear();
                    bombs.clear();
                    ufo.clear();
                    blocks.clear();
                    blocks = make_blocks_array(&resources).await;
                    enemies.clear();
                    enemies = make_enemies_array().await;
                    game.score = 0;
                    game.lives = 3;
                    game.mission = 1;
                    game.enemy_speed = resources::ENEMY_INIT_SPEED;
                    game_state = GameState::InitLevel;
                }
            },
        }

        // GC
        match enemies.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                enemies.remove(idx);
            },
            None => {},
        };

        match blocks.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                blocks.remove(idx);
            },
            None => {},
        };

        match bullets.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                bullets.remove(idx);
            },
            None => {},
        };

        match bombs.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                bombs.remove(idx);
            },
            None => {},
        };

        match ufo.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                ufo.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}