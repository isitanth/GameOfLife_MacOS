mod game;
mod ui;

use eframe::egui;
use ui::GameOfLifeApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Conway's Game of Life"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Conway's Game of Life",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            Box::new(GameOfLifeApp::new(cc))
        }),
    )
}
