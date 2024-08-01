mod game;

use game::Game;
use minifb::{Window, WindowOptions, Key};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const CELL_SIZE: usize = 4;

fn main() {
    let mut game = Game::new();
    let mut window = Window::new(
        "Conway",
        WIDTH * CELL_SIZE,
        HEIGHT * CELL_SIZE,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_millis(50)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        game.update();
        game.render();
        
        window.update_with_buffer(&game.buffer, WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE).unwrap();
    }
}