mod banana_drop;

use ruscii::app::State;
use ruscii::terminal::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameId {
    BananaDrop,
}

// Add new games here to make them available in the menu.
pub const GAMES: &[(GameId, &str)] = &[(GameId::BananaDrop, "Banana Drop")];

pub enum Game {
    BananaDrop(banana_drop::BananaDrop),
}

impl Game {
    pub fn new(id: GameId) -> Self {
        match id {
            GameId::BananaDrop => Self::BananaDrop(banana_drop::BananaDrop::new()),
        }
    }

    // Returns true when the player wants to return to the menu.
    pub fn frame(&mut self, state: &State, window: &mut Window) -> bool {
        match self {
            Self::BananaDrop(game) => game.frame(state, window),
        }
    }
}
