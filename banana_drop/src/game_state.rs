pub mod game_state {

    use rand::Rng;
    use ruscii::spatial::Vec2;
    pub struct Bowl {
        pub pos: Vec2,
    }

    pub struct Banana {
        pub pos: Vec2,
        pub speed: f64,
    }

    pub struct GameState {
        pub dimension: Vec2,
        pub bowl: Bowl,
        pub score: u32,
        pub lives: u32,
        pub level: u32,
        pub bananas: Vec<Banana>,
        pub frame_count: u32,
        pub remaining_bananas: u32,
    }

    impl GameState {
        pub fn new() -> Self {
            GameState {
                dimension: Vec2::xy(100, 30),
                bowl: Bowl {
                    pos: Vec2::xy(100 / 2, 32),
                },
                score: 0,
                lives: 5,
                level: 1,
                bananas: Vec::new(),
                frame_count: 0,
                remaining_bananas: 15,
            }
        }

        pub fn reset(&mut self) {
            self.score = 0;
            self.lives = 5;
            self.level = 0;
            self.bananas = Vec::new();
            self.frame_count = 0;
            self.remaining_bananas = 15;
        }

        pub fn spawn_bananas(&mut self) {
            // Number of bananas increases with level
            if self.remaining_bananas > 0 {
                let mut rng = rand::thread_rng();
                let x_pos = rng.gen_range(5..90); // Random x position
                self.bananas.push(Banana {
                    pos: Vec2::xy(x_pos, 5), // Start under top horizontal line
                    speed: 1.0,              // Speed increases with level
                });
                self.remaining_bananas -= 1;
            }
        }

        pub fn update_bananas(&mut self) {
            self.bananas.retain_mut(|banana| {
                banana.pos.y += banana.speed as i32; // Move banana down
                banana.pos.y < 40 // Keep bananas within the screen
            });
        }

        pub fn check_collisions(&mut self) {
            self.bananas.retain(|banana| {
                if (banana.pos.x >= self.bowl.pos.x
                    && banana.pos.x < self.bowl.pos.x + self.dimension.x / 12)
                    && (banana.pos.y >= self.bowl.pos.y && banana.pos.y < self.bowl.pos.y + 2)
                {
                    // banana was caught by bowl
                    self.score += 1; // Increase score
                    false // Remove banana
                } else if banana.pos.y >= 35 {
                    // banana reached the bottom
                    self.lives -= 1;
                    false
                } else {
                    true // Keep banana
                }
            });
        }

        pub fn update_state(&mut self) {
            self.frame_count += 1;
            if self.score % 10 == 0 && self.score > 0 {
                self.level += 1; // Increase level every 10 points
                self.remaining_bananas = self.level * 10
            }
            // Spawn bananas periodically (e.g., every few frames)
            if self.frame_count % 30 == 0 {
                self.spawn_bananas();
            }

            if self.frame_count % 20 == 0 {
                self.update_bananas();
            }

            self.check_collisions();
        }
    }
}
