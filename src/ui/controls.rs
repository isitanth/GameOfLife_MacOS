#[derive(Debug, Clone)]
pub struct SimulationControls {
    pub is_playing: bool,
    pub speed: f32,          // Steps per second
    pub zoom: f32,           // Zoom level for the grid
    pub cell_size: f32,      // Size of each cell in pixels
    pub generation: u64,     // Current generation number
    pub design_mode: bool,   // Whether we're in design mode or simulation mode
    pub randomness_density: f32, // Density for random generation (0.0 to 1.0)
    pub grid_width: usize,   // Current grid width
    pub grid_height: usize,  // Current grid height
}

impl Default for SimulationControls {
    fn default() -> Self {
        Self {
            is_playing: false,
            speed: 5.0,          // 5 generations per second
            zoom: 3.0,           // Start zoomed in for better visibility
            cell_size: 3.0,      // 3 pixels per cell for better visibility
            generation: 0,
            design_mode: true,   // Start in design mode
            randomness_density: 0.3, // Default 30% density
            grid_width: 2000,    // Default grid width
            grid_height: 1500,   // Default grid height
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

    pub fn toggle_design_mode(&mut self) {
        self.design_mode = !self.design_mode;
        if self.design_mode {
            // When entering design mode, pause the simulation
            self.is_playing = false;
        }
    }

    #[allow(dead_code)]
    pub fn enter_simulation_mode(&mut self) {
        self.design_mode = false;
    }

    #[allow(dead_code)]
    pub fn enter_design_mode(&mut self) {
        self.design_mode = true;
        self.is_playing = false;
    }
}
