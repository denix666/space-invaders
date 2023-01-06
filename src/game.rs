pub struct Game {
    pub score: i32,
    pub hi_score: i32,
    pub enemy_speed: f32,
    pub lives: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            hi_score: 0,
            enemy_speed: 0.0,
            lives: 0,
        }
    }
}
