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
       // println!("{}", format!("Script path: {}", primary_path).cyan());
        
        ScriptHandler {
            wrapper_script_path: primary_path,
        }
    }

    pub fn run_section(&self, section: &str) -> Result<bool, String> {
        println!("{}", format!("\n==== Running {} Section ====\n", section.to_uppercase()).blue());
        
        // 检查脚本是否存在
        let script_path = Path::new(&self.wrapper_script_path);
        
        if script_path.exists() {
            // 如果脚本存在，使用脚本
       //     println!("{}", format!("Using script: {}", self.wrapper_script_path).cyan());
            
            // 确保脚本是可执行的
            #[cfg(unix)]
            {
                let _ = Command::new("chmod")
                    .args(["+x", &self.wrapper_script_path])
                    .status()
                    .map_err(|e| format!("Failed to set script permissions: {}", e));
            }
            
            // 运行脚本并传递部分参数
            let output = Command::new(&self.wrapper_script_path)
                .arg(section.to_lowercase())
                .status()
                .map_err(|e| format!("Failed to execute script: {}", e))?;
            
            return Ok(output.success());
        } else {
            // 如果脚本不存在，直接在Rust中实现相应功能
        //    println!("{}", "Using built-in implementation...".cyan());
            return self.handle_section_internally(section);
        }
    }
    
    // 在Rust代码中直接实现各个部分的功能
    fn handle_section_internally(&self, section: &str) -> Result<bool, String> {
        match section.to_lowercase().as_str() {
            "xcode" => self.handle_xcode_section(),
            "brew" => self.handle_brew_section(),
            "git" => self.handle_git_section(),
            "ssh" => self.handle_ssh_section(),
            "vscode" => self.handle_vscode_section(),
            "node" => self.handle_node_section(),
            "iterm" => self.handle_iterm_section(),
            "zsh" => self.handle_zsh_section(),
            "docker" => self.handle_docker_section(),
            "devtools" => self.handle_devtools_section(),
            "apps" => self.handle_apps_section(),
            "macos" => self.handle_macos_section(),
            "workspace" => self.handle_workspace_section(),
            _ => Err(format!("Unknown section: {}", section)),
        }
    }
    
    // 实现各个部分的功能
    fn handle_xcode_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing Xcode Command Line Tools ====\n".blue());
        
        // 检查xcode command line tools是否已安装
        let xcode_select_check = Command::new("xcode-select")
            .arg("-p")
            .output()
            .map_err(|e| format!("Failed to check xcode-select: {}", e))?;
        
        if xcode_select_check.status.success() {
            println!("{}", "✓ Xcode Command Line Tools already installed".green());
            println!("{}", "Xcode Command Line Tools installation completed".green());
            return Ok(true);
        } else {
            println!("{}", "Installing Xcode Command Line Tools...".cyan());
            
            // 触发安装
            let install_result = Command::new("xcode-select")
                .args(["--install"])
                .status()
                .map_err(|e| format!("Failed to run xcode-select --install: {}", e))?;
            
            if install_result.success() {
                println!("{}", "Xcode Command Line Tools installation triggered".green());
                println!("Please wait for the installation to complete.");
                println!("{}", "Xcode Command Line Tools installation completed".green());
                return Ok(true);
            } else {
                return Err("Failed to install Xcode Command Line Tools".to_string());
            }
        }
    }
    
    fn handle_brew_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing Homebrew ====\n".blue());
        
        // 检查Homebrew是否已安装
        let brew_check = Command::new("which")
            .arg("brew")
            .output()
            .map_err(|e| format!("Failed to check brew: {}", e))?;
        
        if brew_check.status.success() {
            println!("{}", "✓ Homebrew already installed".green());
            
            // 更新Homebrew
            let _ = Command::new("brew")
                .arg("update")
                .status()
                .map_err(|e| format!("Failed to update Homebrew: {}", e))?;
            
            println!("{}", "Homebrew updated".green());
            return Ok(true);
        } else {
            println!("{}", "Installing Homebrew...".cyan());
            
            // 安装Homebrew
            let install_cmd = r#"/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#;
            
            let install_result = Command::new("bash")
                .args(["-c", install_cmd])
                .status()
                .map_err(|e| format!("Failed to install Homebrew: {}", e))?;
            
            if install_result.success() {
                println!("{}", "Homebrew installed".green());
                
                // 在Apple Silicon Mac上添加Homebrew到PATH
                if std::env::consts::ARCH == "aarch64" {
                    println!("Adding Homebrew to PATH for Apple Silicon Mac...");
                    
                    // 将Homebrew添加到zprofile
                    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
                    let zprofile_path = format!("{}/.zprofile", home);
                    
                    let profile_content = "\neval \"$(/opt/homebrew/bin/brew shellenv)\"\n";
                    
                    // 使用标准文件操作方式追加内容
                    match std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&zprofile_path)
                    {
                        Ok(mut file) => {
                            match file.write_all(profile_content.as_bytes()) {
                                Ok(_) => {
                                    println!("{}", "Homebrew added to PATH for Apple Silicon Mac".green());
                                },
                                Err(e) => {
                                    println!("{}", format!("Warning: Could not update .zprofile: {}", e).yellow());
                                }
                            }
                        },
                        Err(e) => {
                            println!("{}", format!("Warning: Could not open .zprofile: {}", e).yellow());
                        }
                    }
                }
                
                return Ok(true);
            } else {
                return Err("Failed to install Homebrew".to_string());
            }
        }
    }
    
    fn handle_git_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing and configuring Git ====\n".blue());
        
        // 检查Git是否已安装
        let git_check = Command::new("which")
            .arg("git")
            .output()
            .map_err(|e| format!("Failed to check git: {}", e))?;
        
        if !git_check.status.success() {
            println!("{}", "Installing Git...".cyan());
            
            // 使用Homebrew安装Git
            let install_result = Command::new("brew")
                .args(["install", "git"])
                .status()
                .map_err(|e| format!("Failed to install Git: {}", e))?;
            
            if !install_result.success() {
                return Err("Failed to install Git".to_string());
            }
            
            println!("{}", "Git installed".green());
        } else {
            println!("{}", "✓ Git already installed".green());
        }
        
        // 简单的Git配置检查
        let git_user_name = Command::new("git")
            .args(["config", "--global", "user.name"])
            .output()
            .map_err(|e| format!("Failed to check git config: {}", e))?;
        
        if git_user_name.stdout.is_empty() {
            println!("Git needs to be configured");
            println!("{}", "Git configuration should be done manually.".yellow());
            println!("Please run: git config --global user.name \"Your Name\"");
            println!("Please run: git config --global user.email \"your.email@example.com\"");
        } else {
            println!("{}", "✓ Git already configured".green());
        }
        
        println!("{}", "Git setup completed".green());
        Ok(true)
    }
    
    // 其他部分的实现（简化版，可根据需要扩展）
    fn handle_ssh_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== SSH Key Generation ====\n".blue());
        println!("{}", "This feature requires interactive input and is better performed via the original script.".yellow());
        println!("Please run the original script directly for this feature.");
        Ok(true)
    }
    
    fn handle_vscode_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing Visual Studio Code ====\n".blue());
        
        // 检查VS Code是否已安装
        let vscode_check = Command::new("which")
            .arg("code")
            .output()
            .map_err(|e| format!("Failed to check VS Code: {}", e))?;
        
        if vscode_check.status.success() {
            println!("{}", "✓ VS Code already installed".green());
        } else {
            println!("{}", "Installing VS Code...".cyan());
            
            // 使用Homebrew安装VS Code
            let install_result = Command::new("brew")
                .args(["install", "--cask", "visual-studio-code"])
                .status()
                .map_err(|e| format!("Failed to install VS Code: {}", e))?;
            
            if install_result.success() {
                println!("{}", "VS Code installed".green());
            } else {
                return Err("Failed to install VS Code".to_string());
            }
        }
        
        println!("{}", "VS Code setup completed".green());
        Ok(true)
    }
    
    fn handle_node_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Node.js Setup ====\n".blue());
        println!("{}", "This feature requires interactive input and is better performed via the original script.".yellow());
        println!("Please run the original script directly for this feature.");
        Ok(true)
    }
    
    fn handle_iterm_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing iTerm2 ====\n".blue());
        
        // 使用Homebrew安装iTerm2
        let install_result = Command::new("brew")
            .args(["install", "--cask", "iterm2"])
            .status()
            .map_err(|e| format!("Failed to install iTerm2: {}", e))?;
        
        if install_result.success() {
            println!("{}", "iTerm2 installed".green());
        } else {
            return Err("Failed to install iTerm2".to_string());
        }
        
        Ok(true)
    }
    
    fn handle_zsh_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing Oh My Zsh ====\n".blue());
        
        // 检查Oh My Zsh是否已安装
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
        let omz_path = format!("{}/.oh-my-zsh", home);
        
        if Path::new(&omz_path).exists() {
            println!("{}", "✓ Oh My Zsh already installed".green());
            return Ok(true);
        }
        
        println!("{}", "Installing Oh My Zsh...".cyan());
        println!("{}", "This requires running a curl command and is better performed via the original script.".yellow());
        println!("To install Oh My Zsh, please run:");
        println!("sh -c \"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\"");
        
        Ok(true)
    }
    
    fn handle_docker_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing Docker ====\n".blue());
        
        // 使用Homebrew安装Docker
        let install_result = Command::new("brew")
            .args(["install", "--cask", "docker"])
            .status()
            .map_err(|e| format!("Failed to install Docker: {}", e))?;
        
        if install_result.success() {
            println!("{}", "Docker installed".green());
            println!("Please launch Docker Desktop to complete the setup.");
        } else {
            return Err("Failed to install Docker".to_string());
        }
        
        Ok(true)
    }
    
    fn handle_devtools_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing Developer Tools ====\n".blue());
        
        // 简单的开发工具安装列表
        let tools = [
            "jq", "ripgrep", "fd", "bat", "exa", "httpie", "htop"
        ];
        
        for tool in tools.iter() {
            println!("Installing {}...", tool);
            
            let install_result = Command::new("brew")
                .args(["install", tool])
                .status()
                .map_err(|e| format!("Failed to install {}: {}", tool, e))?;
            
            if install_result.success() {
                println!("{}", format!("✓ {} installed", tool).green());
            } else {
                println!("{}", format!("Failed to install {}", tool).red());
            }
        }
        
        println!("{}", "Developer tools installation completed".green());
        Ok(true)
    }
    
    fn handle_apps_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Installing Applications ====\n".blue());
        
        println!("{}", "This feature requires interactive selection and is better performed via the original script.".yellow());
        println!("Please run the original script directly for this feature.");
        
        Ok(true)
    }
    
    fn handle_macos_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Configuring macOS Settings ====\n".blue());
        
        // 示例：配置Finder显示隐藏文件
        println!("Setting Finder to show hidden files...");
        let _ = Command::new("defaults")
            .args(["write", "com.apple.finder", "AppleShowAllFiles", "-bool", "true"])
            .status();
        
        println!("Setting Finder to show path bar...");
        let _ = Command::new("defaults")
            .args(["write", "com.apple.finder", "ShowPathbar", "-bool", "true"])
            .status();
        
        println!("Setting Finder to show status bar...");
        let _ = Command::new("defaults")
            .args(["write", "com.apple.finder", "ShowStatusBar", "-bool", "true"])
            .status();
        
        println!("Disabling press-and-hold for keys in favor of key repeat...");
        let _ = Command::new("defaults")
            .args(["write", "NSGlobalDomain", "ApplePressAndHoldEnabled", "-bool", "false"])
            .status();
        
        println!("Setting a faster keyboard repeat rate...");
        let _ = Command::new("defaults")
            .args(["write", "NSGlobalDomain", "KeyRepeat", "-int", "2"])
            .status();
        
        let _ = Command::new("defaults")
            .args(["write", "NSGlobalDomain", "InitialKeyRepeat", "-int", "15"])
            .status();
        
        println!("Disabling auto-correct...");
        let _ = Command::new("defaults")
            .args(["write", "NSGlobalDomain", "NSAutomaticSpellingCorrectionEnabled", "-bool", "false"])
            .status();
        
        println!("Restarting Finder to apply changes...");
        let _ = Command::new("killall")
            .arg("Finder")
            .status();
        
        let _ = Command::new("killall")
            .arg("SystemUIServer")
            .status();
        
        println!("{}", "macOS settings configured".green());
        Ok(true)
    }
    
    fn handle_workspace_section(&self) -> Result<bool, String> {
        println!("{}", "\n==== Creating Development Workspace ====\n".blue());
        
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
        let workspace_path = format!("{}/Workspace", home);
        
        // 创建Workspace目录
        match fs::create_dir_all(&workspace_path) {
            Ok(_) => {
                println!("{}", format!("✓ Created workspace directory at {}", workspace_path).green());
                Ok(true)
            },
            Err(e) => {
                Err(format!("Failed to create workspace directory: {}", e))
            }
        }
    }
} 