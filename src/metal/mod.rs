use objc2::runtime::ProtocolObject;
use objc2::{msg_send_id, ClassType};
use objc2_foundation::{NSData, NSString};
use objc2_metal::*;
use std::ptr::NonNull;
use bytemuck::{Pod, Zeroable};
use anyhow::{anyhow, Result};
use std::time::Instant;

pub mod buffer_pool;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GridDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct MetalRenderer {
    device: ProtocolObject<dyn MTLDevice>,
    command_queue: ProtocolObject<dyn MTLCommandQueue>,
    compute_pipeline: ProtocolObject<dyn MTLComputePipelineState>,
    shared_compute_pipeline: Option<ProtocolObject<dyn MTLComputePipelineState>>,
    buffer_pool: buffer_pool::BufferPool,
    threadgroup_size: MTLSize,
    max_threadgroups: MTLSize,
}

impl MetalRenderer {
    pub fn new() -> Result<Self> {
        let device = Self::create_device()?;
        let command_queue = Self::create_command_queue(&device)?;
        let library = Self::load_metal_library(&device)?;
        let compute_pipeline = Self::create_compute_pipeline(&device, &library, "evolve_game_of_life")?;
        let shared_compute_pipeline = Self::create_compute_pipeline(&device, &library, "evolve_game_of_life_shared").ok();
        
        // Optimize threadgroup size for M1 Pro (16 GPU cores)
        // Use 8x8 threadgroups for good occupancy
        let threadgroup_size = MTLSize {
            width: 8,
            height: 8, 
            depth: 1,
        };
        
        // Calculate max threadgroups based on M1 Pro capabilities
        let max_threadgroups = MTLSize {
            width: 4096 / 8,  // Max grid width / threadgroup width
            height: 4096 / 8, // Max grid height / threadgroup height  
            depth: 1,
        };
        
        let buffer_pool = buffer_pool::BufferPool::new(device.clone())?;
        
        Ok(Self {
            device,
            command_queue,
            compute_pipeline,
            shared_compute_pipeline,
            buffer_pool,
            threadgroup_size,
            max_threadgroups,
        })
    }
    
    fn create_device() -> Result<ProtocolObject<dyn MTLDevice>> {
        // Get the default Metal device (should be M1 Pro integrated GPU)
        let device = unsafe { MTLCreateSystemDefaultDevice() }
            .ok_or_else(|| anyhow!("Failed to create Metal device"))?;
        
        println!("Metal Device: {}", unsafe { device.name() });
        println!("  - Unified Memory: {}", unsafe { device.hasUnifiedMemory() });
        println!("  - Max Threadgroup Memory: {} KB", unsafe { device.maxThreadgroupMemoryLength() } / 1024);
        println!("  - Max Buffer Length: {} MB", unsafe { device.maxBufferLength() } / (1024 * 1024));
        
        Ok(device)
    }
    
    fn create_command_queue(device: &ProtocolObject<dyn MTLDevice>) -> Result<ProtocolObject<dyn MTLCommandQueue>> {
        unsafe { device.newCommandQueue() }
            .ok_or_else(|| anyhow!("Failed to create Metal command queue"))
    }
    
    fn load_metal_library(device: &ProtocolObject<dyn MTLDevice>) -> Result<ProtocolObject<dyn MTLLibrary>> {
        // Load the compiled metal library from build output
        let library_path = env!("METAL_LIBRARY_PATH");
        if library_path.is_empty() {
            return Err(anyhow!("Metal library not compiled - only available on macOS"));
        }
        
        let library_data = std::fs::read(library_path)
            .map_err(|e| anyhow!("Failed to read Metal library: {}", e))?;
        
        let ns_data = unsafe { NSData::dataWithBytes_length(library_data.as_ptr().cast(), library_data.len()) };
        
        let library = unsafe { device.newLibraryWithData_error(&ns_data) }
            .map_err(|e| anyhow!("Failed to create Metal library: {:?}", e))?;
        
        Ok(library)
    }
    
    fn create_compute_pipeline(
        device: &ProtocolObject<dyn MTLDevice>,
        library: &ProtocolObject<dyn MTLLibrary>,
        function_name: &str,
    ) -> Result<ProtocolObject<dyn MTLComputePipelineState>> {
        let function_name_ns = NSString::from_str(function_name);
        let function = unsafe { library.newFunctionWithName(&function_name_ns) }
            .ok_or_else(|| anyhow!("Failed to find Metal function: {}", function_name))?;
        
        let pipeline = unsafe { device.newComputePipelineStateWithFunction_error(&function) }
            .map_err(|e| anyhow!("Failed to create compute pipeline: {:?}", e))?;
        
        println!("Created Metal compute pipeline: {}", function_name);
        println!("  - Max total threads per threadgroup: {}", unsafe { pipeline.maxTotalThreadsPerThreadgroup() });
        println!("  - Threadgroup memory length: {} bytes", unsafe { pipeline.threadgroupMemoryLength() });
        
        Ok(pipeline)
    }
    
    pub fn evolve_grid(&mut self, grid_data: &[u8], width: usize, height: usize) -> Result<Vec<u8>> {
        let start_time = Instant::now();
        
        let buffer_size = width * height;
        
        // Get buffers from pool for better performance
        let current_buffer = self.buffer_pool.get_buffer(buffer_size, MTLResourceOptions::StorageModeShared)?;
        let next_buffer = self.buffer_pool.get_buffer(buffer_size, MTLResourceOptions::StorageModeShared)?;
        
        // Copy grid data to Metal buffer
        unsafe {
            std::ptr::copy_nonoverlapping(
                grid_data.as_ptr(),
                current_buffer.contents().cast(),
                buffer_size,
            );
        }
        
        // Create dimensions buffer
        let dimensions = GridDimensions {
            width: width as u32,
            height: height as u32,
        };
        let dimensions_data = bytemuck::bytes_of(&dimensions);
        let dimensions_buffer = self.buffer_pool.get_buffer(dimensions_data.len(), MTLResourceOptions::StorageModeShared)?;
        unsafe {
            std::ptr::copy_nonoverlapping(
                dimensions_data.as_ptr(),
                dimensions_buffer.contents().cast(),
                dimensions_data.len(),
            );
        }
        
        // Create command buffer
        let command_buffer = unsafe { self.command_queue.commandBuffer() }
            .ok_or_else(|| anyhow!("Failed to create command buffer"))?;
        
        // Create compute encoder
        let compute_encoder = unsafe { command_buffer.computeCommandEncoder() }
            .ok_or_else(|| anyhow!("Failed to create compute encoder"))?;
        
        // Choose pipeline based on grid size - use shared memory for larger grids
        let use_shared_memory = width >= 64 && height >= 64;
        let pipeline = if use_shared_memory && self.shared_compute_pipeline.is_some() {
            self.shared_compute_pipeline.as_ref().unwrap()
        } else {
            &self.compute_pipeline
        };
        
        unsafe {
            compute_encoder.setComputePipelineState(pipeline);
            compute_encoder.setBuffer_offset_atIndex(Some(&current_buffer), 0, 0);
            compute_encoder.setBuffer_offset_atIndex(Some(&next_buffer), 0, 1);
            compute_encoder.setBuffer_offset_atIndex(Some(&dimensions_buffer), 0, 2);
        }
        
        // Calculate threadgroup and grid sizes
        let grid_size = MTLSize {
            width,
            height,
            depth: 1,
        };
        
        let threadgroup_size = if use_shared_memory {
            // For shared memory version, make sure threadgroup size fits in shared memory
            let max_size = 8; // 8x8 fits well in threadgroup memory
            MTLSize {
                width: max_size.min(width),
                height: max_size.min(height),
                depth: 1,
            }
        } else {
            self.threadgroup_size
        };
        
        unsafe {
            compute_encoder.dispatchThreads_threadsPerThreadgroup(grid_size, threadgroup_size);
            compute_encoder.endEncoding();
        }
        
        // Commit and wait
        unsafe {
            command_buffer.commit();
            command_buffer.waitUntilCompleted();
        }
        
        // Copy result back
        let mut result = vec![0u8; buffer_size];
        unsafe {
            std::ptr::copy_nonoverlapping(
                next_buffer.contents().cast::<u8>(),
                result.as_mut_ptr(),
                buffer_size,
            );
        }
        
        // Return buffers to pool
        self.buffer_pool.return_buffer(current_buffer);
        self.buffer_pool.return_buffer(next_buffer);
        self.buffer_pool.return_buffer(dimensions_buffer);
        
        let gpu_time = start_time.elapsed().as_micros();
        if gpu_time > 1000 { // Only log if > 1ms
            println!("Metal GPU compute: {}μs for {}x{} grid ({} pipeline)", 
                gpu_time, width, height,
                if use_shared_memory { "shared" } else { "basic" }
            );
        }
        
        Ok(result)
    }
    
    pub fn device_info(&self) -> String {
        unsafe {
            format!(
                "Metal Device: {}\n  Unified Memory: {}\n  Max Threads/Threadgroup: {}\n  Threadgroup Memory: {} KB",
                self.device.name(),
                self.device.hasUnifiedMemory(),
                self.compute_pipeline.maxTotalThreadsPerThreadgroup(),
                self.device.maxThreadgroupMemoryLength() / 1024
            )
        }
    }
}