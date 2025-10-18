use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shaders/");
    println!("cargo:rustc-check-cfg=cfg(metal_available)");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let shader_path = "shaders/game_of_life.metal";
    let air_path = format!("{}/game_of_life.air", out_dir);
    let metallib_path = format!("{}/game_of_life.metallib", out_dir);
    
    // Only compile on macOS with Metal toolchain available
    if cfg!(target_os = "macos") {
        // Check if Metal toolchain is available
        let xcrun_check = Command::new("xcrun")
            .arg("--find")
            .arg("metal")
            .output();
            
        match xcrun_check {
            Ok(output) if output.status.success() => {
                // Metal toolchain available, proceed with compilation
                println!("cargo:warning=Metal toolchain found, compiling shaders...");
                
                // Compile .metal to .air (intermediate representation)
                let metal_output = Command::new("xcrun")
                    .arg("metal")
                    .arg("-c")
                    .arg(shader_path)
                    .arg("-o")
                    .arg(&air_path)
                    .arg("-target")
                    .arg("air64-apple-macos11.0") // Target macOS 11.0+ for M1 support
                    .arg("-ffast-math") // Enable fast math for better GPU performance
                    .arg("-O2") // Optimization level 2
                    .output();
                
                match metal_output {
                    Ok(output) if output.status.success() => {
                        // Metal compilation successful, now create metallib
                        let metallib_output = Command::new("xcrun")
                            .arg("metallib")
                            .arg(&air_path)
                            .arg("-o")
                            .arg(&metallib_path)
                            .output();
                        
                        match metallib_output {
                            Ok(output) if output.status.success() => {
                                // Success! Set the library path and enable Metal feature
                                println!("cargo:rustc-env=METAL_LIBRARY_PATH={}", metallib_path);
                                println!("cargo:rustc-cfg=metal_available");
                                println!("Metal shaders compiled successfully!");
                                println!("  - AIR: {}", air_path);
                                println!("  - MetalLib: {}", metallib_path);
                            }
                            Ok(output) => {
                                println!(
                                    "cargo:warning=Metal library compilation failed. CPU-only mode will be used.\nstdout: {}\nstderr: {}",
                                    String::from_utf8_lossy(&output.stdout),
                                    String::from_utf8_lossy(&output.stderr)
                                );
                                println!("cargo:rustc-env=METAL_LIBRARY_PATH=");
                            }
                            Err(e) => {
                                println!("cargo:warning=Failed to execute xcrun metallib: {}. CPU-only mode will be used.", e);
                                println!("cargo:rustc-env=METAL_LIBRARY_PATH=");
                            }
                        }
                    }
                    Ok(output) => {
                        println!(
                            "cargo:warning=Metal shader compilation failed. CPU-only mode will be used.\nstdout: {}\nstderr: {}",
                            String::from_utf8_lossy(&output.stdout),
                            String::from_utf8_lossy(&output.stderr)
                        );
                        println!("cargo:rustc-env=METAL_LIBRARY_PATH=");
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to execute xcrun metal: {}. CPU-only mode will be used.", e);
                        println!("cargo:rustc-env=METAL_LIBRARY_PATH=");
                    }
                }
            }
            _ => {
                println!("cargo:warning=Metal toolchain not found. Install Xcode Command Line Tools for GPU acceleration.");
                println!("cargo:warning=Building with CPU-only mode.");
                println!("cargo:rustc-env=METAL_LIBRARY_PATH=");
            }
        }
    } else {
        // On non-macOS platforms, create a dummy environment variable
        println!("cargo:rustc-env=METAL_LIBRARY_PATH=");
        println!("cargo:warning=Metal shaders not compiled on non-macOS platform");
    }
}