mod game_state;
mod helpers;

use game_state::game_state::GameState;
use helpers::helpers::keyboard_event_handlers;
use ruscii::app::State;
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

pub struct BananaDrop {
    state: GameState,
}

impl BananaDrop {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }

    pub fn frame(&mut self, app_state: &State, window: &mut Window) -> bool {
        let game_state = &mut self.state;
        if keyboard_event_handlers(app_state, game_state) {
            return true;
        }

        if game_state.lives > 0 {
            game_state.update_state();

            for banana in &game_state.bananas {
                Pencil::new(window.canvas_mut())
                    .set_foreground(Color::Yellow)
                    .draw_text("0", banana.pos);
            }

            Pencil::new(window.canvas_mut())
                .set_foreground(Color::White)
                .draw_text("Press Q or Esc for menu", Vec2::xy(1, 1))
                .draw_text("Move with arrow keys <-  ->", Vec2::xy(1, 2))
                .set_foreground(Color::Green)
                .draw_text(&format!("Score : {}", game_state.score), Vec2::xy(1, 3))
                .set_foreground(Color::Red)
                .draw_text(&format!("Lives : {}", game_state.lives), Vec2::xy(1, 4))
                .set_foreground(Color::Blue)
                .draw_rect(
                    &RectCharset::double_lines(),
                    Vec2::xy(4, 5),
                    Vec2::xy(100, 30),
                )
                .set_foreground(Color::Red)
                .draw_rect(
                    &RectCharset::simple_round_lines(),
                    game_state.bowl.pos,
                    Vec2::xy(game_state.dimension.x / 12, 2),
                );
        } else {
            Pencil::new(window.canvas_mut())
                .set_foreground(Color::Red)
                .draw_text("Game over", Vec2::xy(1, 2))
                .set_foreground(Color::Green)
                .draw_text(&format!("Score : {}", game_state.score), Vec2::xy(1, 3))
                .set_foreground(Color::White)
                .draw_text("Press R to restart", Vec2::xy(1, 4))
                .draw_text("Press Q or Esc for menu", Vec2::xy(1, 5));
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_starts_fresh() {
        let mut previous = BananaDrop::new();
        previous.state.score = 10;
        previous.state.lives = 0;

        let next = BananaDrop::new();
        assert_eq!(next.state.score, 0);
        assert_eq!(next.state.lives, 5);
        assert!(next.state.bananas.is_empty());
    }
}
