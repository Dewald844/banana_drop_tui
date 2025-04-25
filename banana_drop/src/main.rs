use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

mod game_state;
mod game_state_helpers;

fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();

    let mut game_state = game_state::game_state::GameState::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        game_state_helpers::game_state_helpers::keyboard_event_handlers(app_state, &mut game_state);
        game_state_helpers::game_state_helpers::game_state_update(&mut game_state);
        fps_counter.update();

        for banana in &game_state.bananas {
            Pencil::new(window.canvas_mut())
                .set_foreground(Color::Yellow)
                .draw_text("0", banana.pos); // Represent banana as an emoji or character
        }

        if game_state.lives > 0 {
            Pencil::new(window.canvas_mut())
                .draw_text(&format!(" 𝟶: {}", game_state.score), Vec2::xy(1, 2))
                .draw_text(&format!("<3 {}", game_state.lives), Vec2::xy(10, 2))
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
                    Vec2 {
                        x: (game_state.dimension.x / 12),
                        y: (2),
                    },
                )
                .set_foreground(Color::White);
        } else {
            Pencil::new(window.canvas_mut()).draw_text("Game over", Vec2::xy(10, 10));
        }
    });
}
