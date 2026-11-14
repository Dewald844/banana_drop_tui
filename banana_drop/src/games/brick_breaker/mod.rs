mod game_state;

use game_state::{GameState, Status};
use ruscii::app::State;
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

pub struct BrickBreaker {
    state: GameState,
}

impl BrickBreaker {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }

    pub fn frame(&mut self, app_state: &State, window: &mut Window) -> bool {
        let mut left = false;
        let mut right = false;
        for event in app_state.keyboard().last_key_events() {
            match event {
                KeyEvent::Pressed(Key::Q | Key::Esc) => return true,
                KeyEvent::Pressed(Key::Space) => self.state.launch(),
                KeyEvent::Pressed(Key::Left) => left = true,
                KeyEvent::Pressed(Key::Right) => right = true,
                KeyEvent::Pressed(Key::R)
                    if matches!(self.state.status, Status::Won | Status::Lost) =>
                {
                    self.state = GameState::new();
                }
                _ => {}
            }
        }

        for key in app_state.keyboard().get_keys_down() {
            match key {
                Key::Left => left = true,
                Key::Right => right = true,
                _ => {}
            }
        }
        self.state
            .move_paddle(2 * (i32::from(right) - i32::from(left)));
        self.state.update();
        self.draw(window);
        false
    }

    fn draw(&self, window: &mut Window) {
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil
            .set_foreground(Color::White)
            .draw_text("BRICK BREAKER", Vec2::xy(2, 0))
            .draw_text(
                "Left/Right: move | Space: launch | Q/Esc: menu",
                Vec2::xy(2, 1),
            )
            .set_foreground(Color::Green)
            .draw_text(&format!("Score: {}", self.state.score), Vec2::xy(2, 2))
            .set_foreground(Color::Red)
            .draw_text(&format!("Lives: {}", self.state.lives), Vec2::xy(20, 2))
            .set_foreground(Color::Blue)
            .draw_rect(
                &RectCharset::double_lines(),
                Vec2::xy(game_state::LEFT, game_state::TOP),
                Vec2::xy(
                    game_state::RIGHT - game_state::LEFT + 1,
                    game_state::BOTTOM - game_state::TOP + 1,
                ),
            );

        for brick in &self.state.bricks {
            pencil
                .set_foreground(Color::Yellow)
                .draw_text("[===]", *brick);
        }
        pencil
            .set_foreground(Color::Green)
            .draw_text(
                &"=".repeat(game_state::PADDLE_WIDTH as usize),
                Vec2::xy(self.state.paddle_x, game_state::PADDLE_Y),
            )
            .set_foreground(Color::White)
            .draw_text("O", self.state.ball);

        let message = match self.state.status {
            Status::Ready => "Press Space to launch",
            Status::Playing => "",
            Status::Won => "You won! Press R to play again",
            Status::Lost => "Game over! Press R to try again",
        };
        pencil
            .set_foreground(Color::White)
            .draw_text(message, Vec2::xy(2, 3));
    }
}
