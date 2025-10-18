use super::{Cell, Grid};
use std::time::Instant;

pub struct GameRules;

impl GameRules {
    /// Apply Conway's Game of Life rules:
    /// 1. Any live cell with fewer than two live neighbors dies (underpopulation)
    /// 2. Any live cell with two or three live neighbors lives on to the next generation
    /// 3. Any live cell with more than three live neighbors dies (overpopulation)
    /// 4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction)
    pub fn evolve(grid: &Grid) -> Grid {
        // Try GPU acceleration first if available
        #[cfg(target_os = "macos")]
        {
            #[cfg(metal_available)]
            {
                let start_time = Instant::now();
                if let Some(gpu_result) = grid.evolve_gpu() {
                    let gpu_time = start_time.elapsed().as_micros();
                    if gpu_time > 1000 { // Only log if > 1ms
                        println!("GPU evolution: {}μs for {}x{} grid", 
                            gpu_time, grid.width(), grid.height());
                    }
                    return gpu_result;
                }
            }
        }
        
        // CPU fallback
        let start_time = Instant::now();
        let result = Self::evolve_cpu(grid);
        let cpu_time = start_time.elapsed().as_micros();
        if cpu_time > 5000 { // Only log if > 5ms
            println!("CPU evolution: {}μs for {}x{} grid", 
                cpu_time, grid.width(), grid.height());
        }
        result
    }
    
    /// CPU-based evolution (original implementation)
    pub fn evolve_cpu(grid: &Grid) -> Grid {
        let mut new_grid = Grid::new(grid.width(), grid.height());
        
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let current_cell = grid.get_cell(x, y);
                let live_neighbors = grid.count_live_neighbors(x, y);
                
                let new_cell = match current_cell {
                    Cell::Alive => {
                        match live_neighbors {
                            2 | 3 => Cell::Alive,  // Survival
                            _ => Cell::Dead,       // Death by underpopulation or overpopulation
                        }
                    }
                    Cell::Dead => {
                        match live_neighbors {
                            3 => Cell::Alive,     // Birth
                            _ => Cell::Dead,      // Remains dead
                        }
                    }
                };
                
                new_grid.set_cell(x, y, new_cell);
            }
        }
        
        new_grid
    }
}