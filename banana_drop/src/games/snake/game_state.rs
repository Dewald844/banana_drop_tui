use rand::seq::IteratorRandom;
use ruscii::spatial::Vec2;
use std::collections::VecDeque;

pub const WIDTH: i32 = 24;
pub const HEIGHT: i32 = 16;
const STEP_FRAMES: u8 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ready,
    Playing,
    Lost,
    Won,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> Vec2 {
        match self {
            Self::Up => Vec2::xy(0, -1),
            Self::Down => Vec2::xy(0, 1),
            Self::Left => Vec2::xy(-1, 0),
            Self::Right => Vec2::xy(1, 0),
        }
    }

    fn opposite(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Up, Self::Down)
                | (Self::Down, Self::Up)
                | (Self::Left, Self::Right)
                | (Self::Right, Self::Left)
        )
    }
}

pub struct GameState {
    // The head is at the front and the tail is at the back.
    pub snake: VecDeque<Vec2>,
    pub food: Option<Vec2>,
    pub score: u32,
    pub status: Status,
    direction: Direction,
    pending_direction: Option<Direction>,
    frame: u8,
}

impl GameState {
    pub fn new() -> Self {
        let mut game = Self {
            snake: VecDeque::from([Vec2::xy(12, 8), Vec2::xy(11, 8), Vec2::xy(10, 8)]),
            food: None,
            score: 0,
            status: Status::Ready,
            direction: Direction::Right,
            pending_direction: None,
            frame: 0,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        self.food = (0..HEIGHT)
            .flat_map(|y| (0..WIDTH).map(move |x| Vec2::xy(x, y)))
            .filter(|cell| !self.snake.contains(cell))
            .choose(&mut rand::thread_rng());
        if self.food.is_none() {
            self.status = Status::Won;
        }
    }

    pub fn start(&mut self) {
        if self.status == Status::Ready {
            self.status = Status::Playing;
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if matches!(self.status, Status::Ready | Status::Playing)
            && self.pending_direction.is_none()
            && direction != self.direction
            && !self.direction.opposite(direction)
        {
            // Only one turn per movement prevents a fast double turn from reversing.
            self.pending_direction = Some(direction);
        }
    }

    pub fn update(&mut self) {
        if self.status != Status::Playing {
            return;
        }
        self.frame += 1;
        if self.frame < STEP_FRAMES {
            return;
        }
        self.frame = 0;
        self.step();
    }

    fn step(&mut self) {
        if let Some(direction) = self.pending_direction.take() {
            self.direction = direction;
        }
        let next = self.snake[0] + self.direction.delta();
        let grows = self.food == Some(next);
        // Moving into the current tail is safe only when it moves away.
        let occupied = self
            .snake
            .iter()
            .take(self.snake.len() - usize::from(!grows));
        if next.x < 0
            || next.x >= WIDTH
            || next.y < 0
            || next.y >= HEIGHT
            || occupied.into_iter().any(|segment| *segment == next)
        {
            self.status = Status::Lost;
            return;
        }
        if !grows {
            self.snake.pop_back();
        }
        self.snake.push_front(next);
        if grows {
            self.score += 1;
            self.spawn_food();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_ready_and_food_is_not_on_snake() {
        let mut game = GameState::new();
        assert_eq!(game.status, Status::Ready);
        assert!(!game.snake.contains(&game.food.unwrap()));
        let head = game.snake[0];
        for _ in 0..STEP_FRAMES * 2 {
            game.update();
        }
        assert_eq!(game.snake[0], head);
        game.start();
        for _ in 0..STEP_FRAMES {
            game.update();
        }
        assert_eq!(game.snake[0], head + Vec2::xy(1, 0));
    }

    #[test]
    fn turns_cannot_reverse_or_change_twice_per_step() {
        let mut game = GameState::new();
        game.turn(Direction::Left);
        assert_eq!(game.pending_direction, None);
        game.turn(Direction::Up);
        game.turn(Direction::Left);
        game.step();
        assert_eq!(game.direction, Direction::Up);
        assert_eq!(game.snake[0], Vec2::xy(12, 7));
        game.turn(Direction::Left);
        game.step();
        assert_eq!(game.direction, Direction::Left);
    }

    #[test]
    fn food_grows_snake_and_increases_score() {
        let mut game = GameState::new();
        game.food = Some(Vec2::xy(13, 8));
        game.step();
        assert_eq!(game.snake.len(), 4);
        assert_eq!(game.score, 1);
        assert!(!game.snake.contains(&game.food.unwrap()));
    }

    #[test]
    fn wall_and_body_collisions_end_game() {
        let mut game = GameState::new();
        game.snake = VecDeque::from([Vec2::xy(WIDTH - 1, 0)]);
        game.step();
        assert_eq!(game.status, Status::Lost);

        let mut game = GameState::new();
        game.snake = VecDeque::from([Vec2::xy(2, 2), Vec2::xy(3, 2), Vec2::xy(3, 3)]);
        game.food = Some(Vec2::xy(0, 0));
        game.step();
        assert_eq!(game.status, Status::Lost);
    }

    #[test]
    fn moving_into_vacated_tail_is_allowed() {
        let mut game = GameState::new();
        game.snake = VecDeque::from([
            Vec2::xy(2, 2),
            Vec2::xy(2, 3),
            Vec2::xy(3, 3),
            Vec2::xy(3, 2),
        ]);
        game.food = Some(Vec2::xy(0, 0));
        game.step();
        assert_ne!(game.status, Status::Lost);
        assert_eq!(game.snake[0], Vec2::xy(3, 2));
    }

    #[test]
    fn filling_board_wins_without_spawning_food() {
        let mut game = GameState::new();
        let last_cell = Vec2::xy(1, 0);
        game.snake = std::iter::once(Vec2::xy(0, 0))
            .chain(
                (0..HEIGHT)
                    .flat_map(|y| (0..WIDTH).map(move |x| Vec2::xy(x, y)))
                    .filter(|cell| *cell != Vec2::xy(0, 0) && *cell != last_cell),
            )
            .collect();
        game.food = Some(last_cell);
        game.step();
        assert_eq!(game.score, 1);
        assert_eq!(game.snake.len(), (WIDTH * HEIGHT) as usize);
        assert_eq!(game.food, None);
        assert_eq!(game.status, Status::Won);
    }
}
