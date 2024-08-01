pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 100;
pub const CELL_SIZE: usize = 5;

const COLORS: [u32; 5] = [0xffd6ff, 0xe7c6ff, 0xc8b6ff, 0xb8c0ff, 0xbbd0ff];
const BACKGROUND: u32 = 0x000000;

type PatternFunction = dyn Fn(&mut Game, usize, usize);

pub struct Game {
    grid: Vec<Vec<bool>>,
    pub buffer: Vec<u32>,
    generation: usize,
    paused: bool,
    tick_rate: std::time::Duration,
    last_update: std::time::Instant,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            grid: vec![vec![false; WIDTH]; HEIGHT],
            buffer: vec![BACKGROUND; WIDTH * HEIGHT * CELL_SIZE * CELL_SIZE],
            generation: 0,
            paused: false,
            tick_rate: std::time::Duration::from_millis(70),
            last_update: std::time::Instant::now(),
        };
        game.initialize_grid();
        game
    }

  fn initialize_grid(&mut self) {
    let patterns: Vec<(&PatternFunction, (usize, usize))> = vec![
        // Patrones centrales
        (&Self::flower, (WIDTH / 2, HEIGHT / 2)),
        (&Self::pulsar, (WIDTH / 2, HEIGHT / 2 - 20)),
        (&Self::pulsar, (WIDTH / 2, HEIGHT / 2 + 20)),
        (&Self::pentadecathlon, (WIDTH / 2 - 20, HEIGHT / 2)),
        (&Self::pentadecathlon, (WIDTH / 2 + 20, HEIGHT / 2)),

        // Patrones en las esquinas
        (&Self::glider, (5, 5)),
        (&Self::glider, (WIDTH - 10, 5)),
        (&Self::glider, (5, HEIGHT - 10)),
        (&Self::glider, (WIDTH - 10, HEIGHT - 10)),

        // Patrones en los bordes
        (&Self::blinker, (WIDTH / 4, 5)),
        (&Self::blinker, (3 * WIDTH / 4, HEIGHT - 5)),
        (&Self::block, (5, HEIGHT / 2)),
        (&Self::block, (WIDTH - 5, HEIGHT / 2)),

        // Patrones dispersos
        (&Self::beehive, (WIDTH / 3, HEIGHT / 3)),
        (&Self::beehive, (2 * WIDTH / 3, 2 * HEIGHT / 3)),
        (&Self::loaf, (WIDTH / 4, 3 * HEIGHT / 4)),
        (&Self::loaf, (3 * WIDTH / 4, HEIGHT / 4)),

        // Nuevos patrones
        (&Self::boat, (WIDTH / 5, HEIGHT / 5)),
        (&Self::tub, (4 * WIDTH / 5, 4 * HEIGHT / 5)),
        (&Self::toad, (WIDTH / 6, HEIGHT / 2)),
        (&Self::beacon, (5 * WIDTH / 6, HEIGHT / 2)),
        (&Self::lightweight_spaceship, (WIDTH / 2, HEIGHT / 6)),
        (&Self::middleweight_spaceship, (WIDTH / 2, 5 * HEIGHT / 6)),
        (&Self::heavyweight_spaceship, (WIDTH / 3, HEIGHT / 2)),

        // Formas especiales
        (&Self::heart, (WIDTH / 4, HEIGHT / 4)),
        (&Self::star, (3 * WIDTH / 4, 3 * HEIGHT / 4)),

        // Más naves espaciales para movimiento
        (&Self::glider, (WIDTH / 3, HEIGHT / 4)),
        (&Self::glider, (2 * WIDTH / 3, 3 * HEIGHT / 4)),
        (&Self::lightweight_spaceship, (WIDTH / 4, 2 * HEIGHT / 3)),
        (&Self::middleweight_spaceship, (3 * WIDTH / 4, HEIGHT / 3)),

        // Patrones estáticos adicionales
        (&Self::block, (WIDTH / 8, HEIGHT / 8)),
        (&Self::block, (7 * WIDTH / 8, 7 * HEIGHT / 8)),
        (&Self::beehive, (3 * WIDTH / 4, HEIGHT / 8)),
        (&Self::beehive, (WIDTH / 8, 3 * HEIGHT / 4)),
        (&Self::loaf, (5 * WIDTH / 6, HEIGHT / 6)),
        (&Self::loaf, (WIDTH / 6, 5 * HEIGHT / 6)),
    ];

    for (pattern_func, (x, y)) in patterns {
        pattern_func(self, x, y);
    }
}


    fn flower(&mut self, x: usize, y: usize) {
        let pattern = [
            (0,0), (0,4), (1,1), (1,3), (2,2),
            (3,1), (3,3), (4,0), (4,4),
            (1,2), (2,1), (2,3), (3,2),
        ];
        self.set_cells(x, y, &pattern);
    }

    fn glider(&mut self, x: usize, y: usize) {
        let pattern = [
            (1, 0),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        self.set_cells(x, y, &pattern);
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
        self.set_cells(x, y, &pattern);
    }

    fn pentadecathlon(&mut self, x: usize, y: usize) {
        let pattern = [
            (0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7), (0,8), (0,9),
            (1,0), (1,9),
            (2,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6), (2,7), (2,8), (2,9)
        ];
        self.set_cells(x, y, &pattern);
    }

    fn blinker(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (1, 0), (2, 0)];
        self.set_cells(x, y, &pattern);
    }

    fn block(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (0, 1), (1, 0), (1, 1)];
        self.set_cells(x, y, &pattern);
    }

    fn beehive(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
        self.set_cells(x, y, &pattern);
    }

    fn loaf(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
        self.set_cells(x, y, &pattern);
    }

    fn set_cells(&mut self, x: usize, y: usize, pattern: &[(usize, usize)]) {
        for &(dx, dy) in pattern {
            let nx = (x + dx) % WIDTH;
            let ny = (y + dy) % HEIGHT;
            self.grid[ny][nx] = true;
        }
    }

    pub fn update(&mut self) {
        if self.paused || self.last_update.elapsed() < self.tick_rate {
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
        self.last_update = std::time::Instant::now();
    }

    pub fn render(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if self.grid[y][x] {
                    COLORS[(x / 20 + y / 20 + self.generation / 10) % COLORS.len()]
                } else {
                    BACKGROUND
                };

                self.draw_cell(x, y, color);
            }
        }
    }

    fn draw_cell(&mut self, x: usize, y: usize, color: u32) {
        for dy in 0..CELL_SIZE {
            for dx in 0..CELL_SIZE {
                let px = x * CELL_SIZE + dx;
                let py = y * CELL_SIZE + dy;
                let index = py * WIDTH * CELL_SIZE + px;
                self.buffer[index] = color;
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

    fn boat(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
        self.set_cells(x, y, &pattern);
    }

    fn tub(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (0, 1), (2, 1), (1, 2)];
        self.set_cells(x, y, &pattern);
    }

    fn toad(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
        self.set_cells(x, y, &pattern);
    }

    fn beacon(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)];
        self.set_cells(x, y, &pattern);
    }

    fn lightweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (4, 0), (0, 1), (0, 2), (4, 2), (0, 3), (1, 3), (2, 3), (3, 3)];
        self.set_cells(x, y, &pattern);
    }

    fn middleweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [(2, 0), (4, 0), (1, 1), (5, 1), (0, 2), (0, 3), (5, 3), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4)];
        self.set_cells(x, y, &pattern);
    }

    fn heavyweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [(2, 0), (3, 0), (5, 0), (6, 0), (1, 1), (6, 1), (0, 2), (0, 3), (6, 3), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4)];
        self.set_cells(x, y, &pattern);
    }

    fn heart(&mut self, x: usize, y: usize) {
        let pattern = [
            (1, 0), (3, 0),
            (0, 1), (1, 1), (2, 1), (3, 1), (4, 1),
            (0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
            (1, 3), (2, 3), (3, 3),
            (2, 4)
        ];
        self.set_cells(x, y, &pattern);
    }

    fn star(&mut self, x: usize, y: usize) {
        let pattern = [
            (2, 0),
            (1, 1), (2, 1), (3, 1),
            (0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
            (1, 3), (2, 3), (3, 3),
            (2, 4)
        ];
        self.set_cells(x, y, &pattern);
    }
}