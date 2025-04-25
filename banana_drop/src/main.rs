use game_state::game_state::PowerType;
use helpers::helpers::keyboard_event_handlers;
use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

mod game_state;
mod helpers;

fn main() {

    // Main setup calls
    let mut app = App::default();
    let mut game_state = game_state::game_state::GameState::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        keyboard_event_handlers(app_state, &mut game_state);
        if game_state.lives > 0 {
            game_state.update_state();

            for banana in &game_state.bananas {
                Pencil::new(window.canvas_mut())
                    .set_foreground(Color::Yellow)
                    .draw_text("0", banana.pos);
            }

            for power_up in &game_state.power_ups {
                let mut pencil = Pencil::new(window.canvas_mut());
                match power_up.power_type {
                    PowerType::Extend => {
                        pencil
                            .set_foreground(Color::Blue)
                            .draw_text("=", power_up.pos);
                    }
                    PowerType::OneUp => {
                        pencil
                            .set_foreground(Color::Green)
                            .draw_text("+", power_up.pos);
                    }
                    PowerType::Shrink => {
                        pencil
                            .set_foreground(Color::Red)
                            .draw_text("-", power_up.pos);
                    }
                }
            }

            Pencil::new(window.canvas_mut())
                .set_foreground(Color::White)
                .draw_text("Press Q to quit", Vec2::xy(4, 1))
                .draw_text(&format!("Move with arrow keys <-  ->"), Vec2::xy(4, 2))
                .set_foreground(Color::Green)
                .draw_text(&format!("Score : {}", game_state.score), Vec2::xy(4, 3))
                .set_foreground(Color::Red)
                .draw_text(&format!("Lives : {}", game_state.lives), Vec2::xy(20, 3))
                .set_foreground(Color::White)
                // Game border
                .draw_rect(
                    &RectCharset::simple_round_lines(),
                    Vec2::xy(4, 5),
                    Vec2::xy(100, 30),
                )
                .set_foreground(Color::Red)
                // Bowl
                .draw_rect(
                    &RectCharset::double_lines(),
                    game_state.bowl.pos,
                    Vec2 {
                        x: (game_state.bowl.size),
                        y: (2),
                    },
                )
                .set_foreground(Color::White);
        } else {
            Pencil::new(window.canvas_mut())
                .set_foreground(Color::Red)
                .draw_text("Game over", Vec2::xy(1, 2))
                .set_foreground(Color::Green)
                .draw_text(&format!("Score : {}", game_state.score), Vec2::xy(1, 3))
                .set_foreground(Color::White)
                .draw_text("Press R to restart", Vec2::xy(1, 4))
                .draw_text("Press Q to quit", Vec2::xy(1, 5));
        }
    });

    println!("{:#?}", game_state);
}
