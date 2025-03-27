use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Copy init_wrapper.sh to the build directory
    let src_path = Path::new("init_wrapper.sh");
    let dest_path = Path::new(&out_dir).join("init_wrapper.sh");
    
    if src_path.exists() {
        // Read the content of the source file
        let content = fs::read_to_string(src_path).expect("Could not read init_wrapper.sh");
        
        // Write it to the destination
        fs::write(&dest_path, content).expect("Could not write to output directory");
        
        // Make the script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dest_path).expect("Failed to get file metadata").permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            fs::set_permissions(&dest_path, perms).expect("Failed to set permissions");
        }
        
        // Tell cargo to rerun this if the source file changes
        println!("cargo:rerun-if-changed=init_wrapper.sh");
    } else {
        // If the source file doesn't exist, create a minimal version
        let minimal_script = r#"#!/bin/bash
echo "Error: This is a placeholder script. Please ensure init_wrapper.sh is in the project directory."
exit 1
"#;
        fs::write(&dest_path, minimal_script).expect("Could not write to output directory");
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dest_path).expect("Failed to get file metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dest_path, perms).expect("Failed to set permissions");
        }
    }
    
    // Also copy the original init.sh if it exists in the project root or parent directory
    let original_script_paths = [
        Path::new("init.sh"),
        Path::new("../init.sh")
    ];
    
    for path in original_script_paths.iter() {
        if path.exists() {
            let content = fs::read_to_string(path).expect("Could not read init.sh");
            let dest = Path::new(&out_dir).join("init.sh");
            fs::write(&dest, content).expect("Could not write to output directory");
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&dest).expect("Failed to get file metadata").permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&dest, perms).expect("Failed to set permissions");
            }
            
            println!("cargo:rerun-if-changed={}", path.display());
            break;
        }
    }
} 