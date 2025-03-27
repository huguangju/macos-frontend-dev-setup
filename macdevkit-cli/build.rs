use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Copy init_wrapper.sh to the build directory
    let dest_path = Path::new(&out_dir).join("init_wrapper.sh");
    
    // Use a hardcoded script content rather than depending on file paths
    // This ensures the script is always available, regardless of the installation environment
    let script_content = include_str!("init_wrapper.sh");
    
    // Write it to the destination
    fs::write(&dest_path, script_content).expect("Could not write to output directory");
    
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
    
    // Also copy the original init.sh if it exists in the project root or parent directory
    let original_script_paths = [
        Path::new("init.sh"),
        Path::new("../init.sh")
    ];
    
    let mut found_init_script = false;
    
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
            found_init_script = true;
            break;
        }
    }
    
    // If init.sh doesn't exist in any of the paths, embed a minimal version
    if !found_init_script {
        let minimal_script = r#"#!/bin/bash
echo "Running with embedded minimal init.sh script"
echo "Warning: This is a minimal version. Full functionality requires the complete init.sh."

case "$1" in
  "xcode")
    # Check if xcode command line tools are installed
    if command -v xcode-select &> /dev/null; then
      echo "✓ Xcode Command Line Tools already installed"
      exit 0
    else
      echo "Installing Xcode Command Line Tools..."
      xcode-select --install
      echo "Installation triggered. Please follow the prompts."
      exit 0
    fi
    ;;
  *)
    echo "Section $1 not implemented in minimal script."
    exit 1
    ;;
esac
"#;
        let dest = Path::new(&out_dir).join("init.sh");
        fs::write(&dest, minimal_script).expect("Could not write to output directory");
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dest).expect("Failed to get file metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dest, perms).expect("Failed to set permissions");
        }
    }
} 