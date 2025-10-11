use eframe::egui;
use egui::{Color32, Pos2, Rect, Sense, Vec2};
use std::time::Instant;

use crate::game::{Cell, Grid, GameRules};
use super::controls::SimulationControls;

pub struct GameOfLifeApp {
    grid: Grid,
    controls: SimulationControls,
    last_update: Instant,
    grid_offset: Vec2,
    mouse_drag_start: Option<Pos2>,
}

impl Default for GameOfLifeApp {
    fn default() -> Self {
        Self {
            grid: Grid::new_random(80, 60, 0.3), // Start with a random grid
            controls: SimulationControls::default(),
            last_update: Instant::now(),
            grid_offset: Vec2::ZERO,
            mouse_drag_start: None,
        }
    }
}

impl GameOfLifeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn draw_grid(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let (response, painter) = ui.allocate_painter(available_size, Sense::click_and_drag());
        
        let cell_size = self.controls.effective_cell_size();
        let grid_width = self.grid.width() as f32 * cell_size;
        let grid_height = self.grid.height() as f32 * cell_size;
        
        // Handle mouse interactions
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let relative_pos = pos - response.rect.left_top() - self.grid_offset;
                let grid_x = (relative_pos.x / cell_size) as usize;
                let grid_y = (relative_pos.y / cell_size) as usize;
                
                if !self.controls.is_playing {
                    self.grid.toggle_cell(grid_x, grid_y);
                }
            }
        }

        // Handle panning
        if response.dragged() {
            if let Some(start) = self.mouse_drag_start {
                if let Some(current) = response.interact_pointer_pos() {
                    self.grid_offset += current - start;
                }
            }
            if let Some(pos) = response.interact_pointer_pos() {
                self.mouse_drag_start = Some(pos);
            }
        } else {
            self.mouse_drag_start = None;
        }

        // Handle zoom with scroll
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.scroll_delta.y);
            if scroll_delta > 0.0 {
                self.controls.zoom_in();
            } else if scroll_delta < 0.0 {
                self.controls.zoom_out();
            }
        }

        // Draw background
        painter.rect_filled(response.rect, 0.0, Color32::from_gray(20));
        
        // Calculate visible region
        let start_x = (-self.grid_offset.x / cell_size).floor().max(0.0) as usize;
        let start_y = (-self.grid_offset.y / cell_size).floor().max(0.0) as usize;
        let end_x = ((available_size.x - self.grid_offset.x) / cell_size).ceil().min(self.grid.width() as f32) as usize;
        let end_y = ((available_size.y - self.grid_offset.y) / cell_size).ceil().min(self.grid.height() as f32) as usize;

        // Draw cells
        for y in start_y..end_y {
            for x in start_x..end_x {
                let cell = self.grid.get_cell(x, y);
                if cell.is_alive() {
                    let pos = response.rect.left_top() + self.grid_offset + Vec2::new(x as f32 * cell_size, y as f32 * cell_size);
                    let rect = Rect::from_min_size(pos, Vec2::splat(cell_size - 1.0));
                    painter.rect_filled(rect, 2.0, Color32::from_rgb(100, 255, 100));
                }
            }
        }

        // Draw grid lines if zoomed in enough
        if cell_size > 5.0 {
            for x in start_x..=end_x {
                let line_x = response.rect.left_top().x + self.grid_offset.x + x as f32 * cell_size;
                if line_x >= response.rect.left() && line_x <= response.rect.right() {
                    painter.line_segment(
                        [Pos2::new(line_x, response.rect.top()), Pos2::new(line_x, response.rect.bottom())],
                        egui::Stroke::new(0.5, Color32::from_gray(60))
                    );
                }
            }
            
            for y in start_y..=end_y {
                let line_y = response.rect.left_top().y + self.grid_offset.y + y as f32 * cell_size;
                if line_y >= response.rect.top() && line_y <= response.rect.bottom() {
                    painter.line_segment(
                        [Pos2::new(response.rect.left(), line_y), Pos2::new(response.rect.right(), line_y)],
                        egui::Stroke::new(0.5, Color32::from_gray(60))
                    );
                }
            }
        }
    }

    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Play/Pause button
            let play_text = if self.controls.is_playing { "⏸ Pause" } else { "▶ Play" };
            if ui.button(play_text).clicked() {
                self.controls.toggle_play();
            }

            // Step button (only when paused)
            if !self.controls.is_playing && ui.button("Step").clicked() {
                self.grid = GameRules::evolve(&self.grid);
                self.controls.next_generation();
            }

            ui.separator();

            // Speed control
            ui.label("Speed:");
            if ui.button("−").clicked() {
                self.controls.decrease_speed();
            }
            ui.label(format!("{:.1} gen/s", self.controls.speed));
            if ui.button("+").clicked() {
                self.controls.increase_speed();
            }

            ui.separator();

            // Zoom control
            ui.label("Zoom:");
            if ui.button("−").clicked() {
                self.controls.zoom_out();
            }
            ui.label(format!("{:.1}×", self.controls.zoom));
            if ui.button("+").clicked() {
                self.controls.zoom_in();
            }
            if ui.button("Reset").clicked() {
                self.controls.reset_zoom();
            }

            ui.separator();

            // Clear and randomize
            if ui.button("Clear").clicked() {
                self.grid.clear();
                self.controls.reset();
            }
            if ui.button("Random").clicked() {
                self.grid.randomize(0.3);
                self.controls.reset();
            }

            ui.separator();
            
            // Generation counter
            ui.label(format!("Generation: {}", self.controls.generation));
        });
    }
}

impl eframe::App for GameOfLifeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update simulation
        if self.controls.is_playing {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_update).as_secs_f32();
            
            if elapsed >= self.controls.step_interval() {
                self.grid = GameRules::evolve(&self.grid);
                self.controls.next_generation();
                self.last_update = now;
            }
        }

        // Request repaint for smooth animation
        if self.controls.is_playing {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                self.draw_controls(ui);
                ui.separator();
                self.draw_grid(ui);
            });
        });
    }
}