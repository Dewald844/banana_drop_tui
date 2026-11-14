mod banana_drop;
mod brick_breaker;
mod snake;

use ruscii::app::State;
use ruscii::terminal::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameId {
    BananaDrop,
    BrickBreaker,
    Snake,
}

// Add new games here to make them available in the menu.
pub const GAMES: &[(GameId, &str)] = &[
    (GameId::BananaDrop, "Banana Drop"),
    (GameId::BrickBreaker, "Brick Breaker"),
    (GameId::Snake, "Snake"),
];

pub enum Game {
    BananaDrop(banana_drop::BananaDrop),
    BrickBreaker(brick_breaker::BrickBreaker),
    Snake(snake::Snake),
}

impl Game {
    pub fn new(id: GameId) -> Self {
        match id {
            GameId::BananaDrop => Self::BananaDrop(banana_drop::BananaDrop::new()),
            GameId::BrickBreaker => Self::BrickBreaker(brick_breaker::BrickBreaker::new()),
            GameId::Snake => Self::Snake(snake::Snake::new()),
        }
    }

    // Returns true when the player wants to return to the menu.
    pub fn frame(&mut self, state: &State, window: &mut Window) -> bool {
        match self {
            Self::BananaDrop(game) => game.frame(state, window),
            Self::BrickBreaker(game) => game.frame(state, window),
            Self::Snake(game) => game.frame(state, window),
        }
    }
}
