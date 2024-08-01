use rand::Rng;

pub const WIDTH: usize = 300;
pub const HEIGHT: usize = 300;
pub const CELL_SIZE: usize = 3;

const LAVENDER: u32 = 0x9370DB;

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
            buffer: vec![0; WIDTH * HEIGHT * CELL_SIZE * CELL_SIZE],
            generation: 0,
            paused: false,
        };
        game.initialize_grid();
        game
    }

    fn initialize_grid(&mut self) {
        let mut rng = rand::thread_rng();
        
        for _ in 0..15 {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            match rng.gen_range(0..5) {
                0 => self.gosper_glider_gun(x, y),
                1 => self.pulsar(x, y),
                2 => self.pentadecathlon(x, y),
                3 => self.r_pentomino(x, y),
                _ => self.random_pattern(x, y),
            }
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
                let color = if self.grid[y][x] { LAVENDER } else { 0 };
                
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
                let nx = (x as i32 + dx + WIDTH as i32) % WIDTH as i32;
                let ny = (y as i32 + dy + HEIGHT as i32) % HEIGHT as i32;
                if self.grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    fn gosper_glider_gun(&mut self, x: usize, y: usize) {
        let pattern = [
            (1,5), (1,6), (2,5), (2,6), (11,5), (11,6), (11,7), (12,4), (12,8), (13,3), (13,9),
            (14,3), (14,9), (15,6), (16,4), (16,8), (17,5), (17,6), (17,7), (18,6), (21,3), (21,4),
            (21,5), (22,3), (22,4), (22,5), (23,2), (23,6), (25,1), (25,2), (25,6), (25,7), (35,3),
            (35,4), (36,3), (36,4)
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

    fn r_pentomino(&mut self, x: usize, y: usize) {
        let pattern = [(1,0), (2,0), (0,1), (1,1), (1,2)];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = true;
        }
    }

    fn random_pattern(&mut self, x: usize, y: usize) {
        let mut rng = rand::thread_rng();
        for dy in 0..10 {
            for dx in 0..10 {
                if rng.gen_bool(0.3) {
                    let nx = (x + dx) % WIDTH;
                    let ny = (y + dy) % HEIGHT;
                    self.grid[ny][nx] = true;
                }
            }
        }
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