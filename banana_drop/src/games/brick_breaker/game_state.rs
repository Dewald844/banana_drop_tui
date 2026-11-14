use ruscii::spatial::Vec2;

pub const LEFT: i32 = 2;
pub const RIGHT: i32 = 61;
pub const TOP: i32 = 5;
pub const BOTTOM: i32 = 22;
pub const PADDLE_Y: i32 = BOTTOM - 1;
pub const PADDLE_WIDTH: i32 = 9;
pub const BRICK_WIDTH: i32 = 5;
const STARTING_LIVES: u8 = 3;
const BALL_STEP_FRAMES: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ready,
    Playing,
    Won,
    Lost,
}

pub struct GameState {
    pub paddle_x: i32,
    pub ball: Vec2,
    pub velocity: Vec2,
    pub bricks: Vec<Vec2>,
    pub score: u32,
    pub lives: u8,
    pub status: Status,
    frame: u8,
}

impl GameState {
    pub fn new() -> Self {
        let paddle_x = (LEFT + RIGHT - PADDLE_WIDTH + 1) / 2;
        let mut game = Self {
            paddle_x,
            ball: Vec2::xy(0, 0),
            velocity: Vec2::xy(1, -1),
            bricks: (0..4)
                .flat_map(|row| (0..8).map(move |col| Vec2::xy(7 + col * 6, 8 + row * 2)))
                .collect(),
            score: 0,
            lives: STARTING_LIVES,
            status: Status::Ready,
            frame: 0,
        };
        game.reset_ball();
        game
    }

    fn reset_ball(&mut self) {
        self.ball = Vec2::xy(self.paddle_x + PADDLE_WIDTH / 2, PADDLE_Y - 1);
        self.velocity = Vec2::xy(1, -1);
        self.frame = 0;
    }

    pub fn move_paddle(&mut self, direction: i32) {
        if matches!(self.status, Status::Won | Status::Lost) {
            return;
        }
        self.paddle_x = (self.paddle_x + direction).clamp(LEFT + 1, RIGHT - PADDLE_WIDTH);
        if self.status == Status::Ready {
            self.ball.x = self.paddle_x + PADDLE_WIDTH / 2;
        }
    }

    pub fn launch(&mut self) {
        if self.status == Status::Ready {
            self.status = Status::Playing;
        }
    }

    pub fn update(&mut self) {
        if self.status != Status::Playing {
            return;
        }
        self.frame += 1;
        if self.frame < BALL_STEP_FRAMES {
            return;
        }
        self.frame = 0;
        self.step_ball();
    }

    fn remove_brick_at(&mut self, x: i32, y: i32) -> bool {
        if let Some(index) = self
            .bricks
            .iter()
            .position(|brick| brick.y == y && x >= brick.x && x < brick.x + BRICK_WIDTH)
        {
            self.bricks.remove(index);
            self.score += 10;
            true
        } else {
            false
        }
    }

    fn step_ball(&mut self) {
        let x = self.ball.x;
        let y = self.ball.y;
        let mut dx = self.velocity.x;
        let mut dy = self.velocity.y;

        if x + dx <= LEFT || x + dx >= RIGHT {
            dx = -dx;
        }
        if y + dy <= TOP {
            dy = -dy;
        }

        // Test the horizontal and vertical neighbours before the diagonal so
        // the ball reflects off the correct face, including brick corners.
        let hit_x = self.remove_brick_at(x + dx, y);
        let hit_y = self.remove_brick_at(x, y + dy);
        if hit_x {
            dx = -dx;
        }
        if hit_y {
            dy = -dy;
        }
        let mut blocked_y = hit_y;
        if !hit_x && !hit_y && self.remove_brick_at(x + dx, y + dy) {
            dy = -dy;
            blocked_y = true;
        }

        if self.bricks.is_empty() {
            self.status = Status::Won;
            return;
        }

        if dy > 0 && y + dy >= PADDLE_Y {
            let next_x = x + dx;
            if next_x >= self.paddle_x && next_x < self.paddle_x + PADDLE_WIDTH {
                dy = -dy;
                // Let the player steer by hitting different halves of the paddle.
                if next_x < self.paddle_x + PADDLE_WIDTH / 2 {
                    dx = -1;
                } else if next_x > self.paddle_x + PADDLE_WIDTH / 2 {
                    dx = 1;
                }
            } else {
                self.lives -= 1;
                if self.lives == 0 {
                    self.status = Status::Lost;
                } else {
                    self.status = Status::Ready;
                    self.reset_ball();
                }
                return;
            }
        }

        // A paddle deflection at the edge must still remain inside the walls.
        if x + dx <= LEFT || x + dx >= RIGHT {
            dx = -dx;
        }
        self.velocity = Vec2::xy(dx, dy);
        self.ball = Vec2::xy(
            x + if hit_x { 0 } else { dx },
            y + if blocked_y { 0 } else { dy },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paddle_is_bounded_and_carries_unlaunched_ball() {
        let mut game = GameState::new();
        game.move_paddle(-1000);
        assert_eq!(game.paddle_x, LEFT + 1);
        assert_eq!(game.ball.x, game.paddle_x + PADDLE_WIDTH / 2);
        game.move_paddle(1000);
        assert_eq!(game.paddle_x, RIGHT - PADDLE_WIDTH);
        assert_eq!(game.ball.x, game.paddle_x + PADDLE_WIDTH / 2);
    }

    #[test]
    fn launch_and_wall_bounce() {
        let mut game = GameState::new();
        game.launch();
        game.ball = Vec2::xy(LEFT + 1, TOP + 1);
        game.velocity = Vec2::xy(-1, -1);
        game.step_ball();
        assert_eq!(game.ball, Vec2::xy(LEFT + 2, TOP + 2));
        assert_eq!(game.velocity, Vec2::xy(1, 1));
    }

    #[test]
    fn hitting_bricks_scores_and_wins() {
        let mut game = GameState::new();
        game.bricks = vec![Vec2::xy(10, 10), Vec2::xy(20, 10)];
        game.ball = Vec2::xy(10, 11);
        game.velocity = Vec2::xy(1, -1);
        game.step_ball();
        assert_eq!(game.score, 10);
        assert_eq!(game.bricks.len(), 1);
        assert_eq!(game.velocity.y, 1);
        game.ball = Vec2::xy(20, 11);
        game.velocity = Vec2::xy(1, -1);
        game.step_ball();
        assert_eq!(game.score, 20);
        assert_eq!(game.status, Status::Won);
    }

    #[test]
    fn brick_bounce_does_not_enter_the_opposite_brick() {
        let mut game = GameState::new();
        game.bricks = vec![Vec2::xy(10, 10), Vec2::xy(10, 12)];
        game.ball = Vec2::xy(11, 11);
        game.velocity = Vec2::xy(1, -1);
        game.step_ball();
        assert_eq!(game.ball, Vec2::xy(12, 11));
        assert_eq!(game.velocity, Vec2::xy(1, 1));
        assert_eq!(game.bricks, vec![Vec2::xy(10, 12)]);
        game.step_ball();
        assert_eq!(game.status, Status::Won);
    }

    #[test]
    fn paddle_bounces_ball_and_misses_cost_lives() {
        let mut game = GameState::new();
        game.launch();
        game.ball = Vec2::xy(game.paddle_x + 2, PADDLE_Y - 1);
        game.velocity = Vec2::xy(1, 1);
        game.step_ball();
        assert_eq!(game.velocity, Vec2::xy(-1, -1));
        assert_eq!(game.lives, STARTING_LIVES);

        for remaining in (0..STARTING_LIVES).rev() {
            game.launch();
            game.ball = Vec2::xy(LEFT + 1, PADDLE_Y - 1);
            game.velocity = Vec2::xy(1, 1);
            game.step_ball();
            assert_eq!(game.lives, remaining);
            assert_eq!(
                game.status,
                if remaining == 0 {
                    Status::Lost
                } else {
                    Status::Ready
                }
            );
        }
    }

    #[test]
    fn update_only_moves_after_launch_and_at_step_interval() {
        let mut game = GameState::new();
        let start = game.ball;
        game.update();
        assert_eq!(game.ball, start);
        game.launch();
        for _ in 1..BALL_STEP_FRAMES {
            game.update();
            assert_eq!(game.ball, start);
        }
        game.update();
        assert_ne!(game.ball, start);
    }
}
