mod game_state;

use game_state::{Direction, GameState, Status};
use ruscii::app::State;
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

const LEFT: i32 = 2;
const TOP: i32 = 5;

pub struct Snake {
    state: GameState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_resets_score_and_state() {
        let mut game = Snake::new();
        game.state.score = 10;
        game.state.status = Status::Lost;
        game.state = GameState::new();
        assert_eq!(game.state.score, 0);
        assert_eq!(game.state.status, Status::Ready);
        assert_eq!(game.state.snake.len(), 3);
    }
}

impl Snake {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }

    pub fn frame(&mut self, app_state: &State, window: &mut Window) -> bool {
        for event in app_state.keyboard().last_key_events() {
            match event {
                KeyEvent::Pressed(Key::Q | Key::Esc) => return true,
                KeyEvent::Pressed(Key::Space) => self.state.start(),
                KeyEvent::Pressed(Key::R)
                    if matches!(self.state.status, Status::Lost | Status::Won) =>
                {
                    self.state = GameState::new();
                }
                KeyEvent::Pressed(Key::Up) => self.state.turn(Direction::Up),
                KeyEvent::Pressed(Key::Down) => self.state.turn(Direction::Down),
                KeyEvent::Pressed(Key::Left) => self.state.turn(Direction::Left),
                KeyEvent::Pressed(Key::Right) => self.state.turn(Direction::Right),
                _ => {}
            }
        }
        self.state.update();
        self.draw(window);
        false
    }

    fn draw(&self, window: &mut Window) {
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil
            .set_foreground(Color::White)
            .draw_text("SNAKE", Vec2::xy(2, 0))
            .draw_text("Arrows: turn | Space: start | Q/Esc: menu", Vec2::xy(2, 1))
            .set_foreground(Color::Green)
            .draw_text(&format!("Score: {}", self.state.score), Vec2::xy(2, 2))
            .set_foreground(Color::Blue)
            .draw_rect(
                &RectCharset::double_lines(),
                Vec2::xy(LEFT, TOP),
                Vec2::xy(game_state::WIDTH * 2 + 2, game_state::HEIGHT + 2),
            );
        let position = |cell: Vec2| Vec2::xy(LEFT + 1 + cell.x * 2, TOP + 1 + cell.y);
        if let Some(food) = self.state.food {
            pencil
                .set_foreground(Color::Red)
                .draw_text("@@", position(food));
        }
        for (index, cell) in self.state.snake.iter().enumerate() {
            pencil
                .set_foreground(if index == 0 {
                    Color::Yellow
                } else {
                    Color::Green
                })
                .draw_text(if index == 0 { "OO" } else { "oo" }, position(*cell));
        }
        let message = match self.state.status {
            Status::Ready => "Press Space to start",
            Status::Playing => "",
            Status::Lost => "Game over! Press R to restart",
            Status::Won => "You won! Press R to play again",
        };
        pencil
            .set_foreground(Color::White)
            .draw_text(message, Vec2::xy(2, 3));
    }
}
