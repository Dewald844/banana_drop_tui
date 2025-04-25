pub mod helpers {
    use crate::game_state::game_state::GameState;
    use ruscii::app::State;
    use ruscii::keyboard::{Key, KeyEvent};
    use ruscii::spatial::Vec2;

    fn is_right_most_edge(current_pos: Vec2) -> bool {
        if current_pos.x > 6 {
            return true;
        } else {
            false
        }
    }

    fn is_left_most_edge(current_pos: Vec2) -> bool {
        if current_pos.x < 95 {
            return true;
        } else {
            return false;
        }
    }

    pub fn keyboard_event_handlers(app_state: &State, game_state: &mut GameState) -> () {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::R) => {
                    if game_state.lives == 0 {
                        game_state.reset();
                    }
                }
                KeyEvent::Pressed(Key::Left) => {
                    if is_right_most_edge(game_state.bowl.pos) {
                        game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_sub(3);
                    }
                }
                KeyEvent::Pressed(Key::Right) => {
                    if is_left_most_edge(game_state.bowl.pos) {
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
                    if is_left_most_edge(game_state.bowl.pos) {
                        game_state.bowl.pos.x = game_state.bowl.pos.x.saturating_add(3);
                    }
                }
                _ => (),
            }
        }
    }
}
