use super::Cell;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Cell>,  // Flat array for better cache locality
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::default(); width * height];
        Self {
            cells,
            width,
            height,
        }
    }

    #[allow(dead_code)]
    pub fn new_random(width: usize, height: usize, density: f64) -> Self {
        let mut rng = rand::thread_rng();
        let cells = (0..width * height)
            .map(|_| Cell::from(rng.gen::<f64>() < density))
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
            self.cells[y * self.width + x]
        } else {
            Cell::Dead
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x].toggle();
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
                    let idx = (ny as usize) * self.width + (nx as usize);
                    if self.cells[idx].is_alive() {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::Dead;
        }
    }

    pub fn randomize(&mut self, density: f64) {
        let mut rng = rand::thread_rng();
        for cell in &mut self.cells {
            *cell = Cell::from(rng.gen::<f64>() < density);
        }
    }

    #[allow(dead_code)]
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if new_width == self.width && new_height == self.height {
            return; // No change needed
        }

        let mut new_cells = vec![Cell::Dead; new_width * new_height];
        
        // Copy existing cells to the new grid, preserving patterns where possible
        let copy_width = self.width.min(new_width);
        let copy_height = self.height.min(new_height);
        
        for y in 0..copy_height {
            for x in 0..copy_width {
                let old_idx = y * self.width + x;
                let new_idx = y * new_width + x;
                new_cells[new_idx] = self.cells[old_idx];
            }
        }
        
        self.cells = new_cells;
        self.width = new_width;
        self.height = new_height;
    }
}
