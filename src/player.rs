use macroquad::prelude::*;

use crate::resources::Resources;

pub const MOVE_STEP: f32 = 4.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub rect: Rect,
}

impl Player {
    pub async fn new(resources: &Resources) -> Self {
        Self {
            x: 320.0,
            y: 480.0,
            texture: resources.player_texture,
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Left) {
            if self.x > 0.0 {
                self.x -= MOVE_STEP;
            }
        }

        if is_key_down(KeyCode::Right) {
            if self.x < 630.0 {
                self.x += MOVE_STEP;
            }
        }
    }

    pub fn draw(&mut self) {
        self.update();
        draw_texture(self.texture, self.x, self.y, WHITE);

        self.rect.w = self.texture.width();
        self.rect.h = self.texture.height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }
}
