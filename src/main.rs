#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;

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

fn draw_info(font: Font, score: &str, hi_score: &str) {
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
    draw_text_ex(
        &header_text,
        57.0,
        240.0,
        TextParams {
            font,
            font_size: 70,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        60.0,
        280.0,
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
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

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                game_state = GameState::InitLevel;
            },
            GameState::InitLevel => {
                game.enemy_speed = resources::ENEMY_INIT_SPEED;
                game_state = GameState::Game;
            },
            GameState::Game => {
                draw_info(resources.font, game.score.to_string().as_str(), game.hi_score.to_string().as_str());
                if is_key_down(KeyCode::Space) {
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

                    if bullets.len() > 0 {
                        if let Some(_i) = bullets[0].rect.intersect(enemy.rect) {
                            enemy.destroyed = true;
                            bullets[0].destroyed = true;
                            game.score += 10;
                        }
                    }

                    if let Some(_i) = player.rect.intersect(enemy.rect) {
                        game_state = GameState::LevelFail;
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
                }

                if need_to_pull_down {
                    for enemy in &mut enemies {
                        enemy.y += 12.0;
                    }
                    game.enemy_speed += 0.2;
                }

                for block in &mut blocks {
                    block.draw();
                }

                if game.score > game.hi_score {
                    game.hi_score = game.score;
                }
            },
            GameState::LevelFail => {
                draw_info(resources.font, game.score.to_string().as_str(), game.hi_score.to_string().as_str());
                for enemy in &mut enemies {
                    enemy.draw();
                }
                for block in &mut blocks {
                    block.draw();
                }
                show_text(resources.font, "LEVEL FAIL", "press 'space' to continue");
            },
            GameState::Paused => {

            },
            GameState::GameOver => {
                println!("game over");
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

        next_frame().await
    }
}