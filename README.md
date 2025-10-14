# Game of Life on MacOS

A Rust implementation of Conway's Game of Life with a minimalist GUI interface built using eframe/egui. This application allows you to visualize and interact with the classic cellular automaton in real-time.

## Features

- **Interactive Grid**: Click to toggle cells when the simulation is paused
- **Real-time Simulation**: Watch the Game of Life evolve with customizable speed
- **Zoom & Pan**: Navigate through the grid with mouse wheel zoom and click-and-drag panning
- **Speed Control**: Adjust simulation speed from 0.1 to 100 generations per second
- **Step-by-step Mode**: Manual stepping through generations when paused
- **Random Generation**: Generate random initial patterns
- **Clear Grid**: Reset the grid to an empty state
- **Generation Counter**: Track the current generation number

## Controls

### Mouse
- **Left Click**: Toggle cell state (when paused)
- **Drag**: Pan the view around the grid
- **Scroll Wheel**: Zoom in/out

### Buttons
- **Play/Pause**: Start or stop the simulation
- **Step**: Advance one generation (when paused)
- **Speed −/+**: Decrease/increase simulation speed
- **Zoom −/+**: Zoom out/in
- **Reset**: Reset zoom to 1×
- **Clear**: Clear all cells
- **Random**: Generate random pattern (30% density)

## Building and Running

### Prerequisites
- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Build and Run
```bash
# Clone or navigate to the project directory
cd ~/projects/GoL

# Build and run in debug mode
cargo run

# Build optimized release version
cargo build --release

# Run optimized version
cargo run --release
```

### Dependencies
- `eframe`: Cross-platform GUI framework
- `egui`: Immediate mode GUI library
- `egui_extras`: Additional egui utilities
- `rand`: Random number generation for grid initialization

## Game Rules

Conway's Game of Life follows these simple rules:

1. **Underpopulation**: Any live cell with fewer than 2 live neighbors dies
2. **Survival**: Any live cell with 2 or 3 live neighbors lives on
3. **Overpopulation**: Any live cell with more than 3 live neighbors dies  
4. **Reproduction**: Any dead cell with exactly 3 live neighbors becomes alive

## Project Structure

```
src/
├── main.rs              # Application entry point
├── game/
│   ├── mod.rs          # Game module exports
│   ├── cell.rs         # Cell state and operations
│   ├── grid.rs         # Grid data structure and operations
│   └── rules.rs        # Game of Life evolution rules
└── ui/
    ├── mod.rs          # UI module exports
    ├── app.rs          # Main application and rendering
    └── controls.rs     # Simulation control state
```

## Performance Notes
Note: The Metal rendering backend is still under development.
Depending on grid size, zoom level, and frame synchronization behavior, you may observe increased CPU usage or frame spikes during rendering.

- Grid rendering is optimized to only draw visible cells
- Simulation runs on a separate timer to maintain consistent speed
- Large grids (>200×200) may impact performance on slower hardware

## License

This project is open source. Feel free to use, modify, and distribute.

## Contributing

Contributions are welcome! Some ideas for improvements:
- Save/load patterns
- More initial pattern presets
- Toroidal (wrapping) grid option
- Different rule sets
- Pattern recognition
- Statistics tracking
