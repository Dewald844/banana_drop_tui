use crate::games::GameId;
use ruscii::drawing::Pencil;
use ruscii::keyboard::Key;
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

#[derive(Debug, PartialEq, Eq)]
pub enum MenuAction {
    None,
    Start(GameId),
    Quit,
}

pub struct Menu {
    selected: usize,
    games: &'static [(GameId, &'static str)],
}

impl Menu {
    pub fn new() -> Self {
        Self {
            selected: 0,
            games: crate::games::GAMES,
        }
    }

    pub fn handle_key(&mut self, key: Key) -> MenuAction {
        match key {
            Key::Q | Key::Esc => MenuAction::Quit,
            Key::Up if !self.games.is_empty() => {
                self.selected = (self.selected + self.games.len() - 1) % self.games.len();
                MenuAction::None
            }
            Key::Down if !self.games.is_empty() => {
                self.selected = (self.selected + 1) % self.games.len();
                MenuAction::None
            }
            Key::Enter if !self.games.is_empty() => MenuAction::Start(self.games[self.selected].0),
            _ => MenuAction::None,
        }
    }

    pub fn draw(&self, window: &mut Window) {
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil
            .set_foreground(Color::Yellow)
            .draw_text("GAME MENU", Vec2::xy(2, 2));
        pencil.set_foreground(Color::White).draw_text(
            "Use Up/Down to select, Enter to play, Q or Esc to quit",
            Vec2::xy(2, 4),
        );
        for (index, (_, name)) in self.games.iter().enumerate() {
            let selected = index == self.selected;
            pencil
                .set_foreground(if selected { Color::Green } else { Color::White })
                .draw_text(
                    &format!("{} {}", if selected { ">" } else { " " }, name),
                    Vec2::xy(2, 6 + index as i32),
                );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_starts_selected_game_and_quit_keys_exit() {
        let mut menu = Menu::new();
        assert_eq!(
            menu.handle_key(Key::Enter),
            MenuAction::Start(GameId::BananaDrop)
        );
        assert_eq!(menu.handle_key(Key::Q), MenuAction::Quit);
        assert_eq!(menu.handle_key(Key::Esc), MenuAction::Quit);
        assert_eq!(menu.handle_key(Key::Left), MenuAction::None);
    }

    #[test]
    fn brick_breaker_is_selectable_from_the_real_menu() {
        let mut menu = Menu::new();
        assert_eq!(menu.handle_key(Key::Down), MenuAction::None);
        assert_eq!(
            menu.handle_key(Key::Enter),
            MenuAction::Start(GameId::BrickBreaker)
        );
        assert!(matches!(
            crate::games::Game::new(GameId::BrickBreaker),
            crate::games::Game::BrickBreaker(_)
        ));
    }

    #[test]
    fn navigation_wraps_through_game_list() {
        static GAMES_FOR_TEST: &[(GameId, &str)] = &[
            (GameId::BananaDrop, "First"),
            (GameId::BananaDrop, "Second"),
        ];
        let mut menu = Menu {
            selected: 0,
            games: GAMES_FOR_TEST,
        };
        menu.handle_key(Key::Up);
        assert_eq!(menu.selected, 1);
        menu.handle_key(Key::Down);
        assert_eq!(menu.selected, 0);
        menu.handle_key(Key::Down);
        assert_eq!(menu.selected, 1);
    }

    #[test]
    fn empty_game_list_is_safe() {
        let mut menu = Menu {
            selected: 0,
            games: &[],
        };
        assert_eq!(menu.handle_key(Key::Up), MenuAction::None);
        assert_eq!(menu.handle_key(Key::Down), MenuAction::None);
        assert_eq!(menu.handle_key(Key::Enter), MenuAction::None);
    }
}
