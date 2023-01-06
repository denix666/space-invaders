use macroquad::prelude::*;

const BOMB_ANIMATION_SPEED: i32 = 9;
const BOMB_SPEED: f32 = 300.0;

pub struct Bomb {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub destroyed: bool,
}

impl Bomb {
    pub async fn new(x: f32, y: f32, bomb_type: &str) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 1..=2 {
            let path = format!("assets/images/bombs/{}_{}_bomb.png", bomb_type, i);
            sprites.push(load_texture(&path).await.unwrap());
        }
        
        Self {
            x,
            y,
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(x, y, 8.0, 15.0),
            destroyed: false,
        }
    }

    pub fn update_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > BOMB_ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.texture.len() {
                self.cur_frame = 0;
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.y += dt * BOMB_SPEED;
        if self.y > 500.0 {
            self.destroyed = true;
        }
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        self.update_animation();
        draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);

        self.rect.x = self.x;
        self.rect.y = self.y;
    }
}
