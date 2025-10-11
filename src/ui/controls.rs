#[derive(Debug, Clone)]
pub struct SimulationControls {
    pub is_playing: bool,
    pub speed: f32,          // Steps per second
    pub zoom: f32,           // Zoom level for the grid
    pub cell_size: f32,      // Size of each cell in pixels
    pub generation: u64,     // Current generation number
}

impl Default for SimulationControls {
    fn default() -> Self {
        Self {
            is_playing: false,
            speed: 5.0,          // 5 generations per second
            zoom: 1.0,
            cell_size: 10.0,     // 10 pixels per cell
            generation: 0,
        }
    }
}

impl SimulationControls {
    pub fn step_interval(&self) -> f32 {
        1.0 / self.speed.max(0.1)
    }

    pub fn effective_cell_size(&self) -> f32 {
        self.cell_size * self.zoom
    }

    pub fn increase_speed(&mut self) {
        self.speed = (self.speed * 1.5).min(100.0);
    }

    pub fn decrease_speed(&mut self) {
        self.speed = (self.speed / 1.5).max(0.1);
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.2).min(5.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.2).max(0.1);
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn reset(&mut self) {
        self.generation = 0;
        self.is_playing = false;
    }

    pub fn next_generation(&mut self) {
        self.generation += 1;
    }
}