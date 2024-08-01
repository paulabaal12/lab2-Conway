mod game;

use game::{Game, WIDTH, HEIGHT, CELL_SIZE};
use minifb::{Window, WindowOptions, Key, MouseMode};

fn main() {
    let mut game = Game::new();
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH * CELL_SIZE,
        HEIGHT * CELL_SIZE,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_millis(50)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            game.toggle_pause();
        }

        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            game.clear();
        }

        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            game = Game::new();
        }

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let cell_x = (x as usize) / CELL_SIZE;
            let cell_y = (y as usize) / CELL_SIZE;
            if window.get_mouse_down(minifb::MouseButton::Left) {
                game.add_cell(cell_x, cell_y);
            }
        }

        game.update();
        game.render();
        
        let title = format!("Conway's Game of Life - Gen: {} | Cells: {}", game.get_generation(), game.get_live_cells());
        window.set_title(&title);
        
        window.update_with_buffer(&game.buffer, WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE).unwrap();
    }
}