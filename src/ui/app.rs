use eframe::egui;
use egui::{Color32, Pos2, Rect, Sense, Vec2};
use std::time::Instant;

use crate::game::{Grid, GameRules};
use super::controls::SimulationControls;

pub struct GameOfLifeApp {
    grid: Grid,
    controls: SimulationControls,
    last_update: Instant,
    grid_offset: Vec2,
    mouse_drag_start: Option<Pos2>,
    frame_history: FrameHistory,
}

#[derive(Debug)]
struct FrameHistory {
    frame_times: std::collections::VecDeque<f32>,
    last_frame_time: Instant,
}

impl FrameHistory {
    fn new() -> Self {
        Self {
            frame_times: std::collections::VecDeque::with_capacity(100),
            last_frame_time: Instant::now(),
        }
    }
    
    fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        
        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > 100 {
            self.frame_times.pop_front();
        }
    }
    
    fn fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg_frame_time: f32 = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        1.0 / avg_frame_time.max(0.001)
    }
}

impl Default for GameOfLifeApp {
    fn default() -> Self {
        Self {
            grid: Grid::new(2000, 1500), // Large grid for long-term behavior observation
            controls: SimulationControls::default(),
            last_update: Instant::now(),
            grid_offset: Vec2::ZERO,
            mouse_drag_start: None,
            frame_history: FrameHistory::new(),
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
        let _grid_width = self.grid.width() as f32 * cell_size;
        let _grid_height = self.grid.height() as f32 * cell_size;
        
        // Handle mouse interactions
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let relative_pos = pos - response.rect.left_top() - self.grid_offset;
                let grid_x = (relative_pos.x / cell_size) as usize;
                let grid_y = (relative_pos.y / cell_size) as usize;
                
                // Allow cell toggling in design mode or when paused
                if self.controls.design_mode || !self.controls.is_playing {
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
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
            if scroll_delta > 0.0 {
                self.controls.zoom_in();
            } else if scroll_delta < 0.0 {
                self.controls.zoom_out();
            }
        }

        // Draw background with different colors for design vs simulation mode
        let bg_color = if self.controls.design_mode {
            Color32::from_rgb(20, 25, 20) // Slightly green tint for design mode
        } else {
            Color32::from_gray(20) // Normal gray for simulation mode
        };
        painter.rect_filled(response.rect, 0.0, bg_color);
        
        // Calculate visible region with padding for smooth scrolling
        let padding = (cell_size * 2.0) as usize;
        let start_x = ((-self.grid_offset.x / cell_size).floor().max(0.0) as usize).saturating_sub(padding);
        let start_y = ((-self.grid_offset.y / cell_size).floor().max(0.0) as usize).saturating_sub(padding);
        let end_x = ((available_size.x - self.grid_offset.x) / cell_size).ceil().min(self.grid.width() as f32) as usize + padding;
        let end_y = ((available_size.y - self.grid_offset.y) / cell_size).ceil().min(self.grid.height() as f32) as usize + padding;
        let end_x = end_x.min(self.grid.width());
        let end_y = end_y.min(self.grid.height());

        // Batch alive cells for more efficient rendering
        let mut alive_rects = Vec::new();
        for y in start_y..end_y {
            for x in start_x..end_x {
                let cell = self.grid.get_cell(x, y);
                if cell.is_alive() {
                    let pos = response.rect.left_top() + self.grid_offset + Vec2::new(x as f32 * cell_size, y as f32 * cell_size);
                    let rect = Rect::from_min_size(pos, Vec2::splat(cell_size - 1.0));
                    alive_rects.push(rect);
                }
            }
        }
        
        // Render all alive cells in one batch for better GPU performance
        for rect in alive_rects {
            painter.rect_filled(rect, 2.0, Color32::from_rgb(100, 255, 100));
        }

        // Draw grid lines if zoomed in enough (only visible ones)
        if cell_size > 5.0 {
            let stroke = egui::Stroke::new(0.5, Color32::from_gray(60));
            
            // Batch vertical lines
            let mut vertical_lines = Vec::new();
            for x in start_x..=end_x {
                let line_x = response.rect.left_top().x + self.grid_offset.x + x as f32 * cell_size;
                if line_x >= response.rect.left() && line_x <= response.rect.right() {
                    vertical_lines.push([
                        Pos2::new(line_x, response.rect.top()), 
                        Pos2::new(line_x, response.rect.bottom())
                    ]);
                }
            }
            
            // Batch horizontal lines
            let mut horizontal_lines = Vec::new();
            for y in start_y..=end_y {
                let line_y = response.rect.left_top().y + self.grid_offset.y + y as f32 * cell_size;
                if line_y >= response.rect.top() && line_y <= response.rect.bottom() {
                    horizontal_lines.push([
                        Pos2::new(response.rect.left(), line_y), 
                        Pos2::new(response.rect.right(), line_y)
                    ]);
                }
            }
            
            // Render all lines in batches
            for line in vertical_lines {
                painter.line_segment(line, stroke);
            }
            for line in horizontal_lines {
                painter.line_segment(line, stroke);
            }
        }
    }

    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Mode toggle button
            let mode_text = if self.controls.design_mode { "🎨 Design Mode" } else { "▶ Simulation" };
            let mut mode_button = ui.button(mode_text);
            if self.controls.design_mode {
                mode_button = mode_button.highlight();
            }
            if mode_button.clicked() {
                self.controls.toggle_design_mode();
            }
            
            ui.separator();
            
            // Show simulation controls only when not in design mode
            if !self.controls.design_mode {
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
            } else {
                // Design mode instructions
                ui.label("Click cells to toggle • Design your pattern");
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
            
            ui.label("Density:");
            ui.add(egui::Slider::new(&mut self.controls.randomness_density, 0.0..=1.0)
                .text("%")
                .custom_formatter(|n, _| format!("{:.0}%", n * 100.0)));
                
            if ui.button("Random").clicked() {
                self.grid.randomize(self.controls.randomness_density as f64);
                self.controls.reset();
            }

            ui.separator();
            
            // Generation counter and grid info
            if !self.controls.design_mode {
                ui.label(format!("Generation: {}", self.controls.generation));
            } else {
                ui.label("Ready to simulate");
            }
            
            ui.separator();
            ui.label(format!("Grid: {}×{}", self.grid.width(), self.grid.height()));
            ui.label(format!("FPS: {:.1}", self.frame_history.fps()));
        });
    }
}

impl eframe::App for GameOfLifeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update frame history for FPS monitoring
        self.frame_history.update();
        
        // Update simulation (only when not in design mode)
        if !self.controls.design_mode && self.controls.is_playing {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_update).as_secs_f32();
            
            if elapsed >= self.controls.step_interval() {
                self.grid = GameRules::evolve(&self.grid);
                self.controls.next_generation();
                self.last_update = now;
            }
        }

        // Request repaint for smooth animation and GPU rendering
        if !self.controls.design_mode && self.controls.is_playing {
            ctx.request_repaint();
        } else {
            // Also request repaint for FPS monitoring and smooth interaction in design mode
            ctx.request_repaint_after(std::time::Duration::from_millis(16)); // ~60 FPS
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