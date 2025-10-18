mod game;
mod ui;
#[cfg(all(target_os = "macos", metal_available))]
mod metal;

use eframe::egui;
use ui::GameOfLifeApp;
use game::GameBenchmark;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = std::env::args().collect();
    
    // Check for benchmark mode
    if args.len() > 1 && args[1] == "benchmark" {
        println!("🚀 Game of Life - Metal GPU Acceleration Benchmark");
        println!("================================================\n");
        
        // Run scaling benchmark
        GameBenchmark::scaling_benchmark();
        
        // Run specific size benchmark if provided
        if args.len() >= 5 {
            if let (Ok(width), Ok(height), Ok(generations)) = (
                args[2].parse::<usize>(),
                args[3].parse::<usize>(),
                args[4].parse::<u32>()
            ) {
                println!("\n🎯 Custom Benchmark:");
                GameBenchmark::benchmark_evolution(width, height, generations, 0.3);
            }
        }
        
        return Ok(());
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Game of Life"),
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            // Enable high-performance mode for Apple Silicon
            present_mode: eframe::wgpu::PresentMode::Fifo, // VSync for smooth rendering
            desired_maximum_frame_latency: Some(2), // Low latency for responsiveness
            ..Default::default()
        },
        ..Default::default()
    };
    
    eframe::run_native(
        "Game of Life",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            Ok(Box::new(GameOfLifeApp::new(cc)))
        }),
    )
}
