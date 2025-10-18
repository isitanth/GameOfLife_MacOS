#include <metal_stdlib>
using namespace metal;

// Game of Life evolution compute shader
// Optimized for Apple Silicon M1 Pro with 8x8 threadgroups

kernel void evolve_game_of_life(
    device const uint8_t* current_grid [[buffer(0)]],
    device uint8_t* next_grid [[buffer(1)]],
    constant uint32_t& width [[buffer(2)]],
    constant uint32_t& height [[buffer(3)]],
    uint2 gid [[thread_position_in_grid]]
) {
    // Bounds check
    if (gid.x >= width || gid.y >= height) {
        return;
    }
    
    const uint32_t index = gid.y * width + gid.x;
    const uint32_t x = gid.x;
    const uint32_t y = gid.y;
    
    // Count live neighbors using optimized neighbor lookup
    uint8_t live_neighbors = 0;
    
    // Unrolled neighbor counting for better performance
    // Check all 8 neighbors with bounds checking
    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            if (dx == 0 && dy == 0) continue; // Skip center cell
            
            int32_t nx = (int32_t)x + dx;
            int32_t ny = (int32_t)y + dy;
            
            // Bounds check with early exit
            if (nx >= 0 && ny >= 0 && nx < (int32_t)width && ny < (int32_t)height) {
                uint32_t neighbor_index = (uint32_t)ny * width + (uint32_t)nx;
                live_neighbors += current_grid[neighbor_index];
            }
        }
    }
    
    // Apply Conway's Game of Life rules
    const uint8_t current_cell = current_grid[index];
    uint8_t next_cell;
    
    if (current_cell == 1) {
        // Live cell: survives with 2 or 3 neighbors, dies otherwise
        next_cell = (live_neighbors == 2 || live_neighbors == 3) ? 1 : 0;
    } else {
        // Dead cell: born with exactly 3 neighbors, stays dead otherwise
        next_cell = (live_neighbors == 3) ? 1 : 0;
    }
    
    next_grid[index] = next_cell;
}

// Optimized version using threadgroup shared memory for larger grids
kernel void evolve_game_of_life_shared(
    device const uint8_t* current_grid [[buffer(0)]],
    device uint8_t* next_grid [[buffer(1)]],
    constant uint32_t& width [[buffer(2)]],
    constant uint32_t& height [[buffer(3)]],
    uint2 gid [[thread_position_in_grid]],
    uint2 lid [[thread_position_in_threadgroup]],
    uint2 group_size [[threads_per_threadgroup]]
) {
    // Bounds check
    if (gid.x >= width || gid.y >= height) {
        return;
    }
    
    // Shared memory for threadgroup (10x10 to include halo for 8x8 threads)
    threadgroup uint8_t shared_grid[10][10];
    
    // Load center data
    if (lid.x < group_size.x && lid.y < group_size.y) {
        shared_grid[lid.y + 1][lid.x + 1] = current_grid[gid.y * width + gid.x];
    }
    
    // Load halo data for neighbors
    // Top edge
    if (lid.y == 0 && gid.y > 0) {
        shared_grid[0][lid.x + 1] = current_grid[(gid.y - 1) * width + gid.x];
    }
    // Bottom edge
    if (lid.y == group_size.y - 1 && gid.y < height - 1) {
        shared_grid[group_size.y + 1][lid.x + 1] = current_grid[(gid.y + 1) * width + gid.x];
    }
    // Left edge
    if (lid.x == 0 && gid.x > 0) {
        shared_grid[lid.y + 1][0] = current_grid[gid.y * width + (gid.x - 1)];
    }
    // Right edge
    if (lid.x == group_size.x - 1 && gid.x < width - 1) {
        shared_grid[lid.y + 1][group_size.x + 1] = current_grid[gid.y * width + (gid.x + 1)];
    }
    
    // Corner cases
    if (lid.x == 0 && lid.y == 0 && gid.x > 0 && gid.y > 0) {
        shared_grid[0][0] = current_grid[(gid.y - 1) * width + (gid.x - 1)];
    }
    if (lid.x == group_size.x - 1 && lid.y == 0 && gid.x < width - 1 && gid.y > 0) {
        shared_grid[0][group_size.x + 1] = current_grid[(gid.y - 1) * width + (gid.x + 1)];
    }
    if (lid.x == 0 && lid.y == group_size.y - 1 && gid.x > 0 && gid.y < height - 1) {
        shared_grid[group_size.y + 1][0] = current_grid[(gid.y + 1) * width + (gid.x - 1)];
    }
    if (lid.x == group_size.x - 1 && lid.y == group_size.y - 1 && gid.x < width - 1 && gid.y < height - 1) {
        shared_grid[group_size.y + 1][group_size.x + 1] = current_grid[(gid.y + 1) * width + (gid.x + 1)];
    }
    
    threadgroup_barrier(mem_flags::mem_threadgroup);
    
    // Count neighbors using shared memory
    uint8_t live_neighbors = 0;
    const uint32_t local_x = lid.x + 1;
    const uint32_t local_y = lid.y + 1;
    
    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            if (dx == 0 && dy == 0) continue;
            live_neighbors += shared_grid[local_y + dy][local_x + dx];
        }
    }
    
    // Apply rules
    const uint8_t current_cell = shared_grid[local_y][local_x];
    const uint32_t index = gid.y * width + gid.x;
    
    if (current_cell == 1) {
        next_grid[index] = (live_neighbors == 2 || live_neighbors == 3) ? 1 : 0;
    } else {
        next_grid[index] = (live_neighbors == 3) ? 1 : 0;
    }
}