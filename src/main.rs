mod game;
mod ui;

use eframe::egui;
use ui::GameOfLifeApp;

fn main() -> Result<(), eframe::Error> {
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
