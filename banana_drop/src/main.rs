use ruscii::app::{App, State};
use ruscii::terminal::{Color, Window};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};


struct Bowl {
    pub pos: Vec2
}

struct GameState {
    pub dimension: Vec2,
    pub bowl: Bowl,
    pub score: u32,
    pub lives: u32,
    pub level: u32,
}

impl GameState {
    pub fn new(size : Vec2) -> Self {
        GameState {
            dimension: Vec2::xy(size.x, size.y),
            bowl: Bowl { pos: Vec2::xy(size.x/2, size.y-4) },
            score: 0,
            lives: 3,
            level: 1,
        }
    }
}

fn is_right_most_edge(current_pos : Vec2) -> bool {
    if current_pos.x >= 15 && current_pos.x <= 15 + 1 {
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
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::Left) => {
                    game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_sub(3);
                }
                KeyEvent::Pressed(Key::Right) => {
                    game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_add(3);
                }
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::Left => {
                    if game_state.bowl.pos.x > 15 {
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

        Pencil::new(window.canvas_mut())
            .draw_text(&format!("Press 'Q' or 'Esc' to quit | "), Vec2::xy(1, 0))
            .draw_text(&format!("Use arrow keys to move the bowl"), Vec2::xy(30, 0))
            .draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1))
            .draw_text(&format!("Screen size: {} x {}", game_state.dimension.x, game_state.dimension.y), Vec2::xy(1, 3))
            .draw_text(&format!("Score: {}", game_state.score), Vec2::xy(1, 2))
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