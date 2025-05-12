pub mod game_state {

    use rand::Rng;
    use ruscii::spatial::Vec2;

    #[derive(Debug)]
    pub struct Bowl {
        pub pos: Vec2,
        pub size : i32,
    }

    #[derive(Debug)]
    pub struct Banana {
        pub pos: Vec2,
        pub speed: f64,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum PowerType {
        OneUp,
        Extend,
        Shrink,
    }

    impl PowerType {
        pub fn new() -> PowerType {
            let mut rng = rand::thread_rng();
            let power_index = rng.gen_range(1..4);

            match power_index {
                1 => PowerType::Extend,
                2 => PowerType::OneUp,
                _ => PowerType::Shrink,
            }
        }
    }

    #[derive(Debug)]
    pub struct PowerUp {
        pub power_type: PowerType,
        pub pos: Vec2,
        pub speed: f64,
    }

    #[derive(Debug)]
    pub struct GameState {
        pub bowl: Bowl,
        pub score: u32,
        pub lives: u32,
        pub level: u32,
        pub bananas: Vec<Banana>,
        pub power_ups: Vec<PowerUp>,
        pub frame_count: u32,
        pub remaining_bananas: u32,
        pub remaining_power_ups: u32,
    }

    impl GameState {
        pub fn new() -> Self {
            GameState {
                bowl: Bowl {
                    pos: Vec2::xy(100 / 2, 32),
                    size: 10,
                },
                score: 0,
                lives: 5,
                level: 1,
                bananas: Vec::new(),
                power_ups: Vec::new(),
                frame_count: 0,
                remaining_bananas: 15,
                remaining_power_ups: 0,
            }
        }

        pub fn reset(&mut self) {
            self.score = 0;
            self.lives = 5;
            self.level = 0;
            self.bananas = Vec::new();
            self.power_ups = Vec::new();
            self.frame_count = 0;
            self.remaining_bananas = 15;
        }

        pub fn spawn_power_ups(&mut self) {
            if self.remaining_power_ups > 0 {
                let mut rng = rand::thread_rng();
                let x_pos = rng.gen_range(5..90);
                self.power_ups.push(PowerUp {
                    power_type: PowerType::new(),
                    pos: Vec2::xy(x_pos, 5),
                    speed: 1.0,
                });
                self.remaining_power_ups -= 1;
            }
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

        pub fn update_power_ups(&mut self) {
            self.power_ups.retain_mut(|power_up| {
                power_up.pos.y += power_up.speed as i32;
                power_up.pos.y < 40
            });
        }

        pub fn update_bananas(&mut self) {
            self.bananas.retain_mut(|banana| {
                banana.pos.y += banana.speed as i32; // Move banana down
                banana.pos.y < 40 // Keep bananas within the screen
            });
        }

        pub fn check_power_up_collisions(&mut self) {
            // Extract bowl information before the retain call
            let bowl_pos = self.bowl.pos;
            let bowl_size = self.bowl.size;

            // Use a separate vector to track which types to process
            let mut power_up_actions = Vec::new();

            self.power_ups.retain(|power_up| {
                if Self::is_collision(&bowl_pos, bowl_size, &power_up.pos) {
                    // Store the action to perform after the retain call
                    power_up_actions.push(power_up.power_type);
                    false // Remove power up
                } else {
                    // power up reached the bottom when pos y >= 35
                    power_up.pos.y < 35
                }
            });

            // Process the collected actions
            for power_type in power_up_actions {
                match power_type {
                    PowerType::Extend => self.bowl.size += 5,
                    PowerType::OneUp => self.lives += 1,
                    PowerType::Shrink => self.bowl.size -= 5,
                }
            }
        }

        pub fn check_banana_collisions(&mut self) {
            // Extract bowl information before the retain call
            let bowl_pos = self.bowl.pos;
            let bowl_size = self.bowl.size;
            let mut caught_count = 0;
            let mut missed_count = 0;

            self.bananas.retain(|banana| {
                if Self::is_collision(&bowl_pos, bowl_size, &banana.pos) {
                    // banana was caught by bowl
                    caught_count += 1;
                    false // Remove banana
                } else if banana.pos.y >= 35 {
                    // banana reached the bottom
                    missed_count += 1;
                    false
                } else {
                    true // Keep banana
                }
            });

            // Update scores and lives outside the retain closure
            self.score += caught_count;
            self.lives = self.lives.saturating_sub(missed_count);
        }

        pub fn update_state(&mut self) {
            self.frame_count += 1;
            self.level = (self.score as f64 /10.0) as u32;
            if self.score % 10 == 0 && self.score > 0 {
                self.remaining_power_ups = 1; // Spawn powerup
                self.remaining_bananas = 15
            }

            // Spawn bananas periodically (e.g., every few frames)
            if self.level >= 30 || self.frame_count % (30 - self.level) == 0 {
                self.spawn_bananas();
                self.spawn_power_ups();
            }

            // check for new char's to spawn
            if self.level >= 20 || self.frame_count % (20 - self.level) == 0 {
                self.update_bananas();
                self.update_power_ups();
            }

            self.check_banana_collisions();
            self.check_power_up_collisions();
        }

        // Common collision detection function
        fn is_collision(bowl_pos: &Vec2, bowl_size: i32, obj_pos: &Vec2) -> bool {
            (obj_pos.x >= bowl_pos.x && obj_pos.x < (bowl_pos.x + bowl_size))
                && (obj_pos.y >= bowl_pos.y && obj_pos.y < bowl_pos.y + 2)
        }
    }
}
