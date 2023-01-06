use macroquad::prelude::*;

// window size in pixels
pub const WINDOW_WIDTH: i32 = 700;
pub const WINDOW_HEIGHT: i32 = 550;

pub const ENEMY_INIT_SPEED: f32 = 0.7;

pub struct Resources {
    pub player_texture: Texture2D,
    pub block_texture: Texture2D,
    pub bullet_texture: Texture2D,
    pub font: Font,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            player_texture: load_texture("assets/images/player.png").await.unwrap(),
            block_texture: load_texture("assets/images/block.png").await.unwrap(),
            bullet_texture: load_texture("assets/images/bullet.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
        }
    }
}