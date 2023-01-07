use macroquad::prelude::*;

const UFO_ANIMATION_SPEED: i32 = 7;
const UFO_FLIGHT_SPEED: f32 = 3.0;

pub struct Ufo {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    pub destroyed: bool,
    pub side: String,
}

impl Ufo {
    pub async fn new(from_side: &str) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=4 {
            let path = format!("assets/images/ufo/ufo_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        let x = match from_side {
            "left" => -200.0,
            _ => 700.0,
        };
        
        Self {
            x,
            y: 10.0,
            side: from_side.to_string(),
            texture: sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(x, 10.0, 100.0, 20.0),
            destroyed: false,
        }
    }

    pub fn update_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > UFO_ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.texture.len() {
                self.cur_frame = 0;
            }
        }
    }

    pub fn update(&mut self) {
        match self.side.to_string().as_str() {
            "right" => {
                self.x -= UFO_FLIGHT_SPEED;
                if self.x < 0.0 - self.texture[self.cur_frame].width() {
                    self.destroyed = true;
                }
            },
            _ => {
                self.x += UFO_FLIGHT_SPEED;
                if self.x > 700.0 {
                    self.destroyed = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        self.update_animation();
        self.update();
        draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);

        self.rect.x = self.x;
        self.rect.y = self.y;
    }
}
