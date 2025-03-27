use std::process::Command;
use colored::*;

pub struct ScriptHandler {
    wrapper_script_path: String,
}

impl ScriptHandler {
    pub fn new() -> Self {
        // Use the wrapper script that's copied during build
        let wrapper_script_path = format!("{}/init_wrapper.sh", env!("OUT_DIR"));
        
        ScriptHandler {
            wrapper_script_path,
        }
    }

    pub fn run_section(&self, section: &str) -> Result<bool, String> {
        println!("{}", format!("\n==== Running {} Section ====\n", section.to_uppercase()).blue());
        
        // Make sure the script is executable
        let _ = Command::new("chmod")
            .args(["+x", &self.wrapper_script_path])
            .status();
        
        // Run the script with the section argument
        let output = Command::new(&self.wrapper_script_path)
            .arg(section.to_lowercase())
            .status()
            .map_err(|e| format!("Failed to execute script: {}", e))?;
        
        Ok(output.success())
    }
} 