mod games;
mod menu;

use games::Game;
use menu::{Menu, MenuAction};
use ruscii::app::{App, State};
use ruscii::keyboard::KeyEvent;
use ruscii::terminal::Window;

enum Screen {
    Menu(Menu),
    Playing(Game),
}

fn main() {
    let mut app = App::default();
    let mut screen = Screen::Menu(Menu::new());

    app.run(
        |app_state: &mut State, window: &mut Window| match &mut screen {
            Screen::Menu(menu) => {
                for event in app_state.keyboard().last_key_events() {
                    if let KeyEvent::Pressed(key) = event {
                        match menu.handle_key(*key) {
                            MenuAction::None => {}
                            MenuAction::Start(game) => {
                                screen = Screen::Playing(Game::new(game));
                                return;
                            }
                            MenuAction::Quit => {
                                app_state.stop();
                                return;
                            }
                        }
                    }
                }
                menu.draw(window);
            }
            Screen::Playing(game) => {
                if game.frame(app_state, window) {
                    screen = Screen::Menu(Menu::new());
                }
            }
        },
    );
}
