use rand::Rng;

pub const WIDTH: usize = 300;
pub const HEIGHT: usize = 300;
pub const CELL_SIZE: usize = 3;

const COLORS: [u32; 5] = [0xFFD700, 0x00FF00, 0x0000FF, 0xFF4500, 0xFF1493];  // Amarillo, Verde, Azul, Naranja, Rosa
const BACKGROUND: u32 = 0x000000;

pub struct Game {
    grid: Vec<Vec<bool>>,
    pub buffer: Vec<u32>,
    generation: usize,
    paused: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            grid: vec![vec![false; WIDTH]; HEIGHT],
            buffer: vec![BACKGROUND; WIDTH * HEIGHT * CELL_SIZE * CELL_SIZE],
            generation: 0,
            paused: false,
        };
        game.initialize_grid();
        game
    }

    fn initialize_grid(&mut self) {
        let center_x = WIDTH / 2;
        let center_y = HEIGHT / 2;
        let offset = 50;

        let patterns: Vec<(&dyn Fn(&mut Game, usize, usize), (usize, usize))> = vec![
            (&Game::flower, (center_x, center_y)),
            (&Game::flower, (center_x - offset, center_y - offset)),
            (&Game::flower, (center_x + offset, center_y - offset)),
            (&Game::flower, (center_x - offset, center_y + offset)),
            (&Game::flower, (center_x + offset, center_y + offset)),
            (&Game::pulsar, (center_x, center_y - offset * 2)),
            (&Game::pulsar, (center_x, center_y + offset * 2)),
            (&Game::pentadecathlon, (center_x - offset * 2, center_y)),
            (&Game::pentadecathlon, (center_x + offset * 2, center_y)),
            (&Game::glider, (center_x - offset * 3, center_y - offset * 3)),
            (&Game::glider, (center_x + offset * 3, center_y - offset * 3)),
            (&Game::glider, (center_x - offset * 3, center_y + offset * 3)),
            (&Game::glider, (center_x + offset * 3, center_y + offset * 3)),
        ];

        for &(pattern_func, (x, y)) in &patterns {
            pattern_func(self, x, y);
        }
    }

    fn flower(&mut self, x: usize, y: usize) {
        let pattern = [
            (0,0), (0,4), (1,1), (1,3), (2,2),
            (3,1), (3,3), (4,0), (4,4),
            (1,2), (2,1), (2,3), (3,2),
        ];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = true;
        }
    }

    fn glider(&mut self, x: usize, y: usize) {
        let pattern = [
            (1, 0),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = true;
        }
    }

    fn pulsar(&mut self, x: usize, y: usize) {
        let pattern = [
            (2,0), (3,0), (4,0), (8,0), (9,0), (10,0),
            (0,2), (5,2), (7,2), (12,2),
            (0,3), (5,3), (7,3), (12,3),
            (0,4), (5,4), (7,4), (12,4),
            (2,5), (3,5), (4,5), (8,5), (9,5), (10,5),
            (2,7), (3,7), (4,7), (8,7), (9,7), (10,7),
            (0,8), (5,8), (7,8), (12,8),
            (0,9), (5,9), (7,9), (12,9),
            (0,10), (5,10), (7,10), (12,10),
            (2,12), (3,12), (4,12), (8,12), (9,12), (10,12),
        ];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = true;
        }
    }

    fn pentadecathlon(&mut self, x: usize, y: usize) {
        let pattern = [
            (0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7), (0,8), (0,9),
            (1,0), (1,9),
            (2,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6), (2,7), (2,8), (2,9)
        ];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = true;
        }
    }

    pub fn update(&mut self) {
        if self.paused {
            return;
        }

        let mut new_grid = vec![vec![false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbors = self.count_live_neighbors(x, y);
                new_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        self.grid = new_grid;
        self.generation += 1;
    }

    pub fn render(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if self.grid[y][x] {
                    COLORS[(x / 20 + y / 20 + self.generation / 10) % COLORS.len()]
                } else {
                    BACKGROUND
                };

                for dy in 0..CELL_SIZE {
                    for dx in 0..CELL_SIZE {
                        let index = (y * CELL_SIZE + dy) * WIDTH * CELL_SIZE + (x * CELL_SIZE + dx);
                        self.buffer[index] = color;
                    }
                }
            }
        }
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx + WIDTH as isize) % WIDTH as isize;
                let ny = (y as isize + dy + HEIGHT as isize) % HEIGHT as isize;
                if self.grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn add_cell(&mut self, x: usize, y: usize) {
        if x < WIDTH && y < HEIGHT {
            self.grid[y][x] = true;
        }
    }

    pub fn clear(&mut self) {
        self.grid = vec![vec![false; WIDTH]; HEIGHT];
        self.generation = 0;
    }

    pub fn get_generation(&self) -> usize {
        self.generation
    }

    pub fn get_live_cells(&self) -> usize {
        self.grid.iter().flatten().filter(|&&cell| cell).count()
    }
}

