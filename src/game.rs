use rand::Rng;

pub const WIDTH: usize = 200;
pub const HEIGHT: usize = 200;
pub const CELL_SIZE: usize = 4;

const COLORS: [(u8, u8, u8); 11] = [
    (255, 0, 0),    // Rojo
    (0, 255, 0),    // Verde
    (0, 0, 255),    // Azul
    (255, 255, 0),  // Amarillo
    (255, 0, 255),  // Magenta
    (0, 255, 255),  // Cian
    (255, 128, 0),  // Naranja
    (128, 0, 255),  // Púrpura
    (0, 255, 128),  // Verde lima
    (255, 128, 128),// Rosa
    (128, 128, 255) // Lavanda
];

pub struct Game {
    grid: Vec<Vec<u8>>,
    pub buffer: Vec<u32>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            grid: vec![vec![0; WIDTH]; HEIGHT],
            buffer: vec![0; WIDTH * HEIGHT * CELL_SIZE * CELL_SIZE],
        };
        game.initialize_grid();
        game
    }

    fn initialize_grid(&mut self) {
        let mut rng = rand::thread_rng();
        
        for _ in 0..20 {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            match rng.gen_range(0..7) {
                0 => self.glider(x, y),
                1 => self.blinker(x, y),
                2 => self.beacon(x, y),
                3 => self.pulsar(x, y),
                4 => self.lightweight_spaceship(x, y),
                5 => self.beehive(x, y),
                _ => self.random_cells(x, y),
            }
        }
    }

    pub fn update(&mut self) {
        let mut new_grid = vec![vec![0; WIDTH]; HEIGHT];
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbors = self.count_live_neighbors(x, y);
                new_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    (0, 3) => 1,
                    (age, 2) | (age, 3) if age > 0 => age.saturating_add(1),
                    _ => 0,
                };
            }
        }
        
        self.grid = new_grid;
    }

    pub fn render(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if self.grid[y][x] > 0 {
                    let color_index = (self.grid[y][x] as usize - 1) % COLORS.len();
                    let (r, g, b) = COLORS[color_index];
                    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
                } else {
                    0
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
                let nx = (x as i32 + dx + WIDTH as i32) % WIDTH as i32;
                let ny = (y as i32 + dy + HEIGHT as i32) % HEIGHT as i32;
                if self.grid[ny as usize][nx as usize] > 0 {
                    count += 1;
                }
            }
        }
        count
    }

    // Patrones
    fn glider(&mut self, x: usize, y: usize) {
        let pattern = [(1,0), (2,1), (0,2), (1,2), (2,2)];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = 1;
        }
    }

    fn blinker(&mut self, x: usize, y: usize) {
        let pattern = [(0,0), (1,0), (2,0)];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = 1;
        }
    }

    fn beacon(&mut self, x: usize, y: usize) {
        let pattern = [(0,0), (1,0), (0,1), (3,2), (2,3), (3,3)];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = 1;
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
            self.grid[ny][nx] = 1;
        }
    }

    fn lightweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [(1,0), (4,0), (0,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3)];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = 1;
        }
    }

    fn beehive(&mut self, x: usize, y: usize) {
        let pattern = [(1,0), (2,0), (0,1), (3,1), (1,2), (2,2)];
        for &(dx, dy) in &pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = 1;
        }
    }

    fn random_cells(&mut self, x: usize, y: usize) {
        let mut rng = rand::thread_rng();
        for dy in 0..5 {
            for dx in 0..5 {
                if rng.gen_bool(0.4) {
                    let nx = (x + dx) % WIDTH;
                    let ny = (y + dy) % HEIGHT;
                    self.grid[ny][nx] = 1;
                }
            }
        }
    }
}