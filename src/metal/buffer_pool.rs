use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::collections::HashMap;
use anyhow::{anyhow, Result};

/// Buffer pool for efficient Metal buffer reuse
/// Reduces allocation overhead for frequent GPU operations
pub struct BufferPool {
    device: ProtocolObject<dyn MTLDevice>,
    pools: HashMap<(usize, MTLResourceOptions), Vec<ProtocolObject<dyn MTLBuffer>>>,
    max_pool_size: usize,
}

impl BufferPool {
    pub fn new(device: ProtocolObject<dyn MTLDevice>) -> Result<Self> {
        Ok(Self {
            device,
            pools: HashMap::new(),
            max_pool_size: 8, // Keep up to 8 buffers per size/options combo
        })
    }
    
    pub fn get_buffer(
        &mut self,
        size: usize,
        options: MTLResourceOptions,
    ) -> Result<ProtocolObject<dyn MTLBuffer>> {
        let key = (size, options);
        
        // Try to reuse an existing buffer
        if let Some(pool) = self.pools.get_mut(&key) {
            if let Some(buffer) = pool.pop() {
                // Clear the buffer for reuse
                unsafe {
                    std::ptr::write_bytes(buffer.contents().cast::<u8>(), 0, size);
                }
                return Ok(buffer);
            }
        }
        
        // Create a new buffer if none available
        let buffer = unsafe { self.device.newBufferWithLength_options(size, options) }
            .ok_or_else(|| anyhow!("Failed to create Metal buffer of size {}", size))?;
        
        Ok(buffer)
    }
    
    pub fn return_buffer(&mut self, buffer: ProtocolObject<dyn MTLBuffer>) {
        let size = unsafe { buffer.length() };
        let options = unsafe { buffer.resourceOptions() };
        let key = (size, options);
        
        let pool = self.pools.entry(key).or_insert_with(Vec::new);
        
        // Only keep buffers up to the pool size limit
        if pool.len() < self.max_pool_size {
            pool.push(buffer);
        }
        // Otherwise, buffer is dropped automatically
    }
    
    pub fn clear(&mut self) {
        self.pools.clear();
    }
    
    #[allow(dead_code)]
    pub fn pool_stats(&self) -> String {
        let mut stats = String::from("Buffer Pool Stats:\n");
        let mut total_buffers = 0;
        let mut total_memory = 0;
        
        for ((size, options), pool) in &self.pools {
            let pool_memory = size * pool.len();
            total_buffers += pool.len();
            total_memory += pool_memory;
            
            stats.push_str(&format!(
                "  {} bytes ({:?}): {} buffers, {} KB\n",
                size,
                options,
                pool.len(),
                pool_memory / 1024
            ));
        }
        
        stats.push_str(&format!(
            "Total: {} buffers, {} KB\n",
            total_buffers,
            total_memory / 1024
        ));
        
        stats
    }
}