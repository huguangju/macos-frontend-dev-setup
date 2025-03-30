use std::process::Command;
use std::fs;
use std::path::Path;
use std::io::Write;
use colored::*;

pub struct ScriptHandler {
    wrapper_script_path: String,
}

impl ScriptHandler {
    pub fn new() -> Self {
        // Primary path: Use the wrapper script that's copied during build
        let primary_path = format!("{}/init_wrapper.sh", env!("OUT_DIR"));
        
        // 记录脚本路径用于调试
        println!("{}", format!("Script path: {}", primary_path).cyan());
        
        ScriptHandler {
            wrapper_script_path: primary_path,
        }
    }

    pub fn run_section(&self, section: &str) -> Result<bool, String> {
        println!("{}", format!("\n==== Running {} Section ====\n", section.to_uppercase()).blue());
        
        // 检查脚本是否存在
        let script_path = Path::new(&self.wrapper_script_path);
        
        if !script_path.exists() {
            println!("{}", format!("Warning: Script not found at path: {}", self.wrapper_script_path).yellow());
            // 尝试创建临时脚本
            return self.create_and_run_temp_script(section);
        }
        
        // 确保脚本是可执行的
        #[cfg(unix)]
        {
            let _ = Command::new("chmod")
                .args(["+x", &self.wrapper_script_path])
                .status()
                .map_err(|e| format!("Failed to set script permissions: {}", e));
        }
        
        // 运行脚本并传递部分参数
        println!("{}", format!("Executing script: {} {}", self.wrapper_script_path, section).cyan());
        
        let output = Command::new(&self.wrapper_script_path)
            .arg(section.to_lowercase())
            .status()
            .map_err(|e| format!("Failed to execute script: {}", e))?;
        
        Ok(output.success())
    }
    
    // 创建临时脚本并执行
    fn create_and_run_temp_script(&self, section: &str) -> Result<bool, String> {
        println!("{}", "Creating temporary script...".yellow());
        
        // 嵌入的最小脚本内容
        let minimal_script = r#"#!/bin/bash
echo "Running with temporary init_wrapper.sh script"
echo "Warning: This is a minimal version. Full functionality requires the complete init_wrapper.sh."

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
  "brew")
    # Install Homebrew if not already installed
    if command -v brew &> /dev/null; then
      echo "✓ Homebrew already installed"
      brew update
      echo "Homebrew updated"
      exit 0
    else
      echo "Installing Homebrew..."
      /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
      echo "Homebrew installation attempted"
      exit 0
    fi
    ;;
  *)
    echo "Section $1 not implemented in minimal script."
    exit 1
    ;;
esac
"#;
        
        // 创建临时目录和文件
        let temp_dir = std::env::temp_dir();
        let temp_script_path = temp_dir.join("macdevkit_temp_script.sh");
        
        // 写入脚本内容
        let mut file = fs::File::create(&temp_script_path)
            .map_err(|e| format!("Failed to create temporary script: {}", e))?;
        
        file.write_all(minimal_script.as_bytes())
            .map_err(|e| format!("Failed to write to temporary script: {}", e))?;
        
        // 设置可执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&temp_script_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&temp_script_path, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
        
        // 运行临时脚本
        println!("{}", format!("Executing temporary script: {} {}", temp_script_path.display(), section).cyan());
        
        let output = Command::new(&temp_script_path)
            .arg(section.to_lowercase())
            .status()
            .map_err(|e| format!("Failed to execute temporary script: {}", e))?;
        
        // 删除临时脚本（可选）
        let _ = fs::remove_file(&temp_script_path);
        
        Ok(output.success())
    }
} 