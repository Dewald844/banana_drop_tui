# Banana catch

---

![Demo image](image.jpeg)

---

#### Prerequisites
- Rust and Cargo

---

#### Setup
- clone this repo
- from the repository root, enter the `banana_drop` directory
- run `cargo build` to build or `cargo run` to start the application


---

### Controls

#### Menu
- Up and Down to select a game
- Enter to start the selected game
- Q or Esc to quit the application

#### Banana Drop
- Left and Right arrow keys to move
- Q or Esc to return to the menu
- R to restart after death

#### Brick Breaker
- Use a terminal at least 62 columns wide and 23 rows high
- Left and Right arrow keys to move the paddle
- Space to launch the ball at the start and after losing a life
- Break all bricks to win; you have three lives
- R to restart after winning or losing
- Q or Esc to return to the menu

#### Snake
- Use a terminal at least 52 columns wide and 23 rows high
- Space to start; arrow keys to turn
- Eat food to grow and score; avoid the walls and your own body
- R to restart after losing or winning
- Q or Esc to return to the menu

### Adding games

Each game lives in its own folder under `banana_drop/src/games`. Add the game
module and register its name, identifier, constructor, and frame handler in
`banana_drop/src/games/mod.rs` to make it available from the menu.

### Power ups (TODO)
- "_" Extend bowl
- "+" 1up
