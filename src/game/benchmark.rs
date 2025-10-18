use super::{Grid, GameRules};
use std::time::{Instant, Duration};

pub struct BenchmarkResult {
    pub cpu_time: Duration,
    pub grid_size: (usize, usize),
    pub generations: u32,
    pub cells_per_second: f64,
}

impl BenchmarkResult {
    pub fn new(cpu_time: Duration, width: usize, height: usize, generations: u32) -> Self {
        let total_cells = (width * height * generations as usize) as f64;
        let cells_per_second = total_cells / cpu_time.as_secs_f64();
        
        Self {
            cpu_time,
            grid_size: (width, height),
            generations,
            cells_per_second,
        }
    }
    
    pub fn print_summary(&self) {
        println!("🔬 Performance Benchmark Results:");
        println!("  Grid Size: {}×{} ({} cells)", 
            self.grid_size.0, self.grid_size.1, 
            self.grid_size.0 * self.grid_size.1);
        println!("  Generations: {}", self.generations);
        println!("  Total Time: {:.2}ms", self.cpu_time.as_millis());
        println!("  Avg Time/Generation: {:.2}ms", 
            self.cpu_time.as_millis() as f64 / self.generations as f64);
        println!("  Throughput: {:.0} cells/sec", self.cells_per_second);
        
        // Performance rating based on M1 Pro expectations
        if self.cells_per_second > 100_000_000.0 {
            println!("  Rating: 🚀 Excellent");
        } else if self.cells_per_second > 50_000_000.0 {
            println!("  Rating: ✅ Good");
        } else if self.cells_per_second > 10_000_000.0 {
            println!("  Rating: ⚡ Fair");
        } else {
            println!("  Rating: 🐌 Slow");
        }
    }
}

pub struct GameBenchmark;

impl GameBenchmark {
    /// Run a performance benchmark on a grid with random initial state
    pub fn benchmark_evolution(width: usize, height: usize, generations: u32, density: f64) -> BenchmarkResult {
        println!("🔬 Starting benchmark: {}×{} grid, {} generations, {:.1}% density", 
            width, height, generations, density * 100.0);
        
        // Create initial random grid
        let mut grid = Grid::new_random(width, height, density);
        
        // Warmup
        println!("  Warming up...");
        for _ in 0..3 {
            grid = GameRules::evolve_cpu(&grid);
        }
        
        // Reset to fresh random state
        grid = Grid::new_random(width, height, density);
        
        // Actual benchmark
        println!("  Running benchmark...");
        let start_time = Instant::now();
        
        for _ in 0..generations {
            grid = GameRules::evolve_cpu(&grid);
        }
        
        let cpu_time = start_time.elapsed();
        let result = BenchmarkResult::new(cpu_time, width, height, generations);
        result.print_summary();
        
        result
    }
    
    /// Compare CPU vs GPU performance (when GPU is available)
    #[cfg(all(target_os = "macos", metal_available))]
    pub fn benchmark_cpu_vs_gpu(width: usize, height: usize, generations: u32, density: f64) -> (BenchmarkResult, Option<BenchmarkResult>) {
        println!("🔬 CPU vs GPU Benchmark: {}×{} grid, {} generations", width, height, generations);
        
        let grid = Grid::new_random(width, height, density);
        
        // CPU benchmark
        println!("  Testing CPU performance...");
        let cpu_start = Instant::now();
        let mut cpu_grid = grid.clone();
        for _ in 0..generations {
            cpu_grid = GameRules::evolve_cpu(&cpu_grid);
        }
        let cpu_time = cpu_start.elapsed();
        let cpu_result = BenchmarkResult::new(cpu_time, width, height, generations);
        
        // GPU benchmark (if available)
        if grid.has_gpu_acceleration() {
            println!("  Testing GPU performance...");
            let gpu_start = Instant::now();
            let mut gpu_grid = grid.clone();
            for _ in 0..generations {
                if let Some(evolved) = gpu_grid.evolve_gpu() {
                    gpu_grid = evolved;
                } else {
                    println!("  GPU evolution failed, using CPU fallback");
                    gpu_grid = GameRules::evolve_cpu(&gpu_grid);
                }
            }
            let gpu_time = gpu_start.elapsed();
            let gpu_result = BenchmarkResult::new(gpu_time, width, height, generations);
            
            // Validate results match
            if Self::grids_match(&cpu_grid, &gpu_grid) {
                println!("  ✅ CPU and GPU results match!");
            } else {
                println!("  ❌ CPU and GPU results differ!");
            }
            
            // Print comparison
            let speedup = cpu_time.as_secs_f64() / gpu_time.as_secs_f64();
            println!("  🏁 GPU Speedup: {:.2}×", speedup);
            
            cpu_result.print_summary();
            gpu_result.print_summary();
            
            return (cpu_result, Some(gpu_result));
        }
        
        cpu_result.print_summary();
        (cpu_result, None)
    }
    
    /// Validate that two grids have identical states
    #[allow(dead_code)]
    fn grids_match(grid1: &Grid, grid2: &Grid) -> bool {
        if grid1.width() != grid2.width() || grid1.height() != grid2.height() {
            return false;
        }
        
        for y in 0..grid1.height() {
            for x in 0..grid1.width() {
                if grid1.get_cell(x, y) != grid2.get_cell(x, y) {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Run a series of scaling benchmarks
    pub fn scaling_benchmark() -> Vec<BenchmarkResult> {
        let test_cases = [
            (100, 100, 50),
            (200, 200, 25),
            (400, 400, 12),
            (800, 800, 6),
            (1000, 1000, 5),
        ];
        
        let mut results = Vec::new();
        
        println!("🔬 Scaling Benchmark - Testing different grid sizes");
        
        for (width, height, generations) in test_cases {
            let result = Self::benchmark_evolution(width, height, generations, 0.3);
            results.push(result);
        }
        
        // Summary
        println!("\n📊 Scaling Summary:");
        for result in &results {
            println!("  {}×{}: {:.1}M cells/sec", 
                result.grid_size.0, result.grid_size.1,
                result.cells_per_second / 1_000_000.0);
        }
        
        results
    }
}