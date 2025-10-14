# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Development Commands

### Build & Run
```bash
# Build and run in debug mode (recommended for development)
cargo run

# Build optimized release version
cargo build --release

# Run optimized version (better performance)
cargo run --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Run clippy with all targets and features
cargo clippy --all-targets --all-features
```

### Testing
```bash
# Run all tests (currently no tests exist in this project)
cargo test

# Run tests with output visible
cargo test -- --nocapture
```

## Project Architecture

This is a Conway's Game of Life implementation using Rust with the egui/eframe GUI framework. The architecture follows a clean separation between game logic and UI presentation.

### Core Architecture

**Two-Module System:**
- `game/` - Pure game logic, no GUI dependencies
- `ui/` - GUI presentation layer using egui/eframe

**Game Module (`src/game/`):**
- `Cell` - Enum representing alive/dead states with utility methods
- `Grid` - 2D grid data structure managing the cellular automaton state
- `GameRules` - Implements Conway's Game of Life evolution rules as pure functions

**UI Module (`src/ui/`):**
- `GameOfLifeApp` - Main application implementing eframe::App trait
- `SimulationControls` - State management for simulation parameters (speed, zoom, generation counter)

### Key Design Patterns

**Functional Evolution:** The game rules create a new Grid each generation rather than mutating in place, following functional programming principles.

**Viewport Optimization:** Only visible cells are rendered to maintain performance with large grids. The rendering system includes:
- Pan and zoom functionality with mouse interaction
- Grid line rendering only when zoomed in sufficiently
- Efficient neighbor counting algorithm

**State Separation:** Simulation state is cleanly separated from presentation state, making the core game logic testable and reusable.

### Data Flow

1. **Input Handling:** Mouse clicks toggle cells (when paused), drag for panning, scroll for zoom
2. **Simulation Loop:** Timer-based evolution using `GameRules::evolve()` when playing
3. **Rendering:** Viewport-culled cell rendering with dynamic grid lines

### Performance Characteristics

- Grid size: Optimized for grids up to ~200×200 cells
- Rendering: Only draws cells within the visible viewport
- Memory: Uses `Vec<Vec<Cell>>` for simple 2D grid storage
- Evolution: O(width × height) time complexity per generation

## Development Guidelines

### Code Organization
- Keep game logic in `src/game/` completely independent of UI concerns
- UI code should only handle presentation and user interaction
- Use the established pattern of pure functions for game rule implementation

### Cell Interaction
- Cell toggling only works when simulation is paused (`!controls.is_playing`)
- Use `Grid::toggle_cell()` for interactive cell modification
- Random generation uses 30% density (`0.3`) by default

### Zoom and Performance
- Cell rendering includes 1-pixel spacing (`cell_size - 1.0`)
- Grid lines only render when `cell_size > 5.0` pixels
- Zoom limits: 0.1× to 5.0×, speed limits: 0.1 to 100 gen/sec

### Dependencies
- `eframe` - Cross-platform GUI framework
- `egui` - Immediate mode GUI library  
- `egui_extras` - Additional egui utilities
- `rand` - Random number generation for grid initialization