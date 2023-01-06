use macroquad::prelude::*;

use crate::resources::Resources;

pub struct Block {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub rect: Rect,
    pub destroyed: bool,
}

impl Block {
    pub async fn new(x: f32, y: f32, resources: &Resources) -> Self {
        Self {
            x,
            y,
            texture: resources.block_texture,
            rect: Rect::new(x, y, 10.0, 10.0),
            destroyed: false,
        }
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
