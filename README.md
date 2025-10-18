# Game of Life on MacOS

A Rust implementation of Conway's Game of Life with a minimalist GUI interface built using eframe/egui. This application allows you to visualize and interact with the classic cellular automaton in real-time. Using Metal, a low-level, low-overhead hardware-accelerated 3D graphic and compute shader API created by Apple. In result this is allowing to run the project from a laptop while processing 3M cells in real time.

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

## Building and running

### Prerequisites
- Rust (latest stable version recommended)
- Cargo (comes with Rust)
- **macOS 11.0+** with Apple Silicon (M1/M2) for GPU acceleration
- **Xcode Command Line Tools** for Metal shader compilation (`xcode-select --install`)

### Build and run
```bash
# Clone or navigate to the project directory
cd ~/projects/GoL

# Build and run in debug mode
cargo run

# Build optimized release version
cargo build --release

# Run optimized version
cargo run --release

# Run performance benchmarks
cargo run --release -- benchmark

# Custom benchmark (width height generations)
cargo run --release -- benchmark 1000 1000 10
```

### Dependencies
- `eframe`: Cross-platform GUI framework
- `egui`: Immediate mode GUI library
- `egui_extras`: Additional egui utilities
- `rand`: Random number generation for grid initialization

## Game rules

Conway's Game of Life follows these simple rules:

1. **Underpopulation**: Any live cell with fewer than 2 live neighbors dies
2. **Survival**: Any live cell with 2 or 3 live neighbors lives on
3. **Overpopulation**: Any live cell with more than 3 live neighbors dies  
4. **Reproduction**: Any dead cell with exactly 3 live neighbors becomes alive

## Project structure

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

## Performance notes

### Metal GPU Acceleration 🚀

This implementation includes **Metal GPU compute acceleration** for Apple Silicon (M1/M2) Macs:

- **Automatic GPU Detection**: Detects Metal support and falls back to CPU if unavailable
- **Optimized Compute Shaders**: Two shader variants (basic and shared memory) tuned for M1 Pro
- **Unified Memory Usage**: Leverages Apple Silicon's unified memory architecture
- **Buffer Pool**: Efficient Metal buffer reuse to minimize allocation overhead
- **8×8 Threadgroups**: Optimized for M1 Pro's 16-core GPU tile architecture

### CPU Performance (M1 Pro)
- **135M+ cells/sec** on large grids (1000×1000)
- **Viewport Culling**: Only renders visible cells for smooth 60fps
- **Flat Array Storage**: Cache-friendly memory layout
- **Batch Rendering**: Minimizes GPU draw calls

## License

This project is open source. Feel free to use, modify, and distribute.

## Technical Implementation

### Metal GPU Compute Pipeline
- **Direct Metal API**: Uses `objc2-metal` for direct Metal API access (no wgpu wrapper)
- **Compute Shaders**: Written in Metal Shading Language (.metal files)
- **Build-time Compilation**: Shaders compiled to .metallib during `cargo build`
- **Conditional Compilation**: GPU features only enabled when Metal toolchain is available

### Architecture
```
src/
├── metal/                  # Metal GPU compute implementation
│   ├── mod.rs              # MetalRenderer with compute pipeline
│   └── buffer_pool.rs      # Efficient Metal buffer management  
├── game/
│   ├── grid.rs             # Grid with GPU evolution support
│   ├── rules.rs            # GameRules with GPU/CPU dispatch
│   └── benchmark.rs        # Performance testing utilities
└── shaders/
    └── game_of_life.metal  # Metal compute kernels
```

## Contributing

Contributions are welcome! Some ideas for improvements:
- Save/load patterns
- More initial pattern presets
- Toroidal (wrapping) grid option
- Different rule sets
- Pattern recognition
- Statistics tracking
- Multi-GPU support
- Vulkan compute backend for non-Apple platforms
