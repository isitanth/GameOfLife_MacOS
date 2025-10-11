use super::Cell;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::default(); width]; height];
        Self {
            cells,
            width,
            height,
        }
    }

    pub fn new_random(width: usize, height: usize, density: f64) -> Self {
        let mut rng = rand::thread_rng();
        let cells = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| Cell::from(rng.gen::<f64>() < density))
                    .collect()
            })
            .collect();
        
        Self {
            cells,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        if x < self.width && y < self.height {
            self.cells[y][x]
        } else {
            Cell::Dead
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y][x] = cell;
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[y][x].toggle();
        }
    }

    pub fn count_live_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32 {
                    if self.cells[ny as usize][nx as usize].is_alive() {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::Dead;
            }
        }
    }

    pub fn randomize(&mut self, density: f64) {
        let mut rng = rand::thread_rng();
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::from(rng.gen::<f64>() < density);
            }
        }
    }

    pub fn cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }
}