use ruscii::app::{App, State};
use ruscii::terminal::{Color, Window};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};
use rand::Rng;


struct Bowl {
    pub pos: Vec2
}

struct Banana {
    pub pos: Vec2,
    pub speed: f64,
}

struct GameState {
    pub dimension: Vec2,
    pub bowl: Bowl,
    pub score: u32,
    pub lives: u32,
    pub level: u32,
    pub bananas : Vec<Banana>,
    pub frame_count : u32,
}

impl GameState {
    pub fn new(size : Vec2) -> Self {
        GameState {
            dimension: Vec2::xy(size.x, size.y),
            bowl: Bowl { pos: Vec2::xy(size.x/2, size.y-4) },
            score: 0,
            lives: 3,
            level: 1,
            bananas: Vec::new(),
            frame_count: 0,
        }
    }

    pub fn spawn_bananas(&mut self) {
        let banana_count = self.level; // Number of bananas increases with level
        let mut rng = rand::thread_rng();

        for _ in 0..banana_count {
            let x_pos = rng.gen_range(15..(self.dimension.x - 15)); // Random x position
            self.bananas.push(Banana {
                pos: Vec2::xy(x_pos, 5),  // Start under top horizontal line
                speed: self.level as f64,       // Speed increases with level
            });
        }
    }

    pub fn update_bananas(&mut self) {
        self.bananas.retain_mut(|banana| {
            banana.pos.y += banana.speed as i32; // Move banana down
            banana.pos.y < self.dimension.y      // Keep bananas within the screen
        });
    }

    pub fn check_collisions(&mut self) {
        self.bananas.retain(|banana| {
            if (banana.pos.x >= self.bowl.pos.x && banana.pos.x < self.bowl.pos.x + self.dimension.x / 12)
                && (banana.pos.y >= self.bowl.pos.y && banana.pos.y < self.bowl.pos.y + 2)
            {
                self.score += 1; // Increase score
                false // Remove banana
            } 
            else if banana.pos.y >= (self.bowl.pos.y+4) { // banana reached the bottom
                self.lives -= 1;
                false
            }
            else {
                true // Keep banana
            }
        });
    }
}

fn is_right_most_edge(current_pos : Vec2) -> bool {
    if current_pos.x >= 16 {
        return true;
    }
    false
}

fn is_left_most_edge(current_pos : Vec2, dimension : Vec2) -> bool {
    if current_pos.x < ((dimension.x) - dimension.x/12 - 16) {
        return true;
    } else {
        return false;
    }
}

fn main() {
    
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();

    let mut game_state = GameState::new(app.window().size());

    app.run(|app_state: &mut State, window: &mut Window| {

        game_state.frame_count += 1;

        if game_state.score % 10 == 0 && game_state.score > 0 {
            game_state.level += 1; // Increase level every 10 points
        }

        // Spawn bananas periodically (e.g., every few frames)
        if game_state.frame_count % 30 == 0 { // Adjust spawn rate as needed
            game_state.spawn_bananas();
        }

        game_state.update_bananas();

        game_state.check_collisions();

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::Left) => {
                    if is_right_most_edge(game_state.bowl.pos) {
                        game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_sub(3);
                    }
                }
                KeyEvent::Pressed(Key::Right) => {
                    if is_left_most_edge(game_state.bowl.pos, game_state.dimension) {
                        game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_add(3);
                    }
                }
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::Left => {
                    if is_right_most_edge(game_state.bowl.pos) {
                        game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_sub(3);
                    }
                }
                Key::Right => {
                    if is_left_most_edge(game_state.bowl.pos, game_state.dimension) {
                        game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_add(3);
                    }
                }
                _ => (),
            }
        }

        fps_counter.update();

        for banana in &game_state.bananas {
            Pencil::new(window.canvas_mut())
                .set_foreground(Color::Yellow)
                .draw_text("🍌", banana.pos); // Represent banana as an emoji or character
        }

        Pencil::new(window.canvas_mut())
            .draw_text(&format!("＄: {}", game_state.score), Vec2::xy(1, 2))
            .draw_text(&format!("❤️ {}", game_state.lives), Vec2::xy(10, 2))
            .set_foreground(Color::White)
            .draw_hline('\'', Vec2 { x: (15), y: (4) }, game_state.dimension.x - 30)
            .set_foreground(Color::Red)
            .draw_rect(
                &RectCharset::simple_round_lines(),
                game_state.bowl.pos,
                Vec2 { x: (game_state.dimension.x/12), y:(2) },
            ) 
            .set_foreground(Color::White)
            .draw_hline('\'', Vec2 { x: (15), y: (game_state.dimension.y-2) }, game_state.dimension.x - 30);
    });
}