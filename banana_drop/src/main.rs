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

    let mut game_state = game_state::game_state::GameState::new(app.window().size());

    app.run(|app_state: &mut State, window: &mut Window| {
        game_state_helpers::game_state_helpers::keyboard_event_handlers(app_state, &mut game_state);
        game_state_helpers::game_state_helpers::game_state_update(&mut game_state);

        fps_counter.update();

        for banana in &game_state.bananas {
            Pencil::new(window.canvas_mut())
                .set_foreground(Color::Yellow)
                .draw_text("🍌", banana.pos); // Represent banana as an emoji or character
        }

        Pencil::new(window.canvas_mut())
            .draw_text(&format!("Score: {}", game_state.score), Vec2::xy(1, 2))
            .draw_text(&format!("Lives {}", game_state.lives), Vec2::xy(10, 2))
            .set_foreground(Color::White)
            .draw_hline('\'', Vec2 { x: (15), y: (4) }, game_state.dimension.x - 30)
            .set_foreground(Color::Red)
            .draw_rect(
                &RectCharset::simple_round_lines(),
                game_state.bowl.pos,
                Vec2 {
                    x: (game_state.dimension.x / 12),
                    y: (2),
                },
            )
            .set_foreground(Color::White)
            .draw_hline(
                '\'',
                Vec2 {
                    x: (15),
                    y: (game_state.dimension.y - 2),
                },
                game_state.dimension.x - 30,
            );
    });
}
