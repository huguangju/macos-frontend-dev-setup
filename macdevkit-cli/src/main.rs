use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::process::Command;
use which::which;

mod script_handler;
use script_handler::ScriptHandler;

#[derive(Parser)]
#[command(name = "macdevkit")]
#[command(about = "MacDevKit: A comprehensive setup tool for macOS development environments", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the full setup with interactive prompts
    Setup,
    /// Install Xcode Command Line Tools
    Xcode,
    /// Install Homebrew
    Brew,
    /// Install and configure Git
    Git,
    /// Generate SSH key
    Ssh,
    /// Install Visual Studio Code
    Vscode,
    /// Install Node.js via NVM
    Node,
    /// Install iTerm2
    Iterm,
    /// Install Oh My Zsh
    Zsh,
    /// Install Docker
    Docker,
    /// Install additional developer tools
    DevTools,
    /// Install useful applications
    Apps,
    /// Configure macOS settings
    MacOS,
    /// Create development workspace
    Workspace,
}

fn main() {
    print_welcome();
    
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Setup) => run_full_setup(),
        Some(Commands::Xcode) => install_xcode_tools(),
        Some(Commands::Brew) => install_homebrew(),
        Some(Commands::Git) => install_git(),
        Some(Commands::Ssh) => generate_ssh_key(),
        Some(Commands::Vscode) => install_vscode(),
        Some(Commands::Node) => install_node(),
        Some(Commands::Iterm) => install_iterm(),
        Some(Commands::Zsh) => install_oh_my_zsh(),
        Some(Commands::Docker) => install_docker(),
        Some(Commands::DevTools) => install_dev_tools(),
        Some(Commands::Apps) => install_apps(),
        Some(Commands::MacOS) => configure_macos(),
        Some(Commands::Workspace) => create_workspace(),
        None => run_interactive_menu(),
    }
}

fn print_welcome() {
    println!("{}", r#"
    __  ___          ____             __ __ _ __ 
   /  |/  /___ _____/ __ \___ _   __/ //_/(_) /_
  / /|_/ / __ `/ __/ / / / _ \ | / / ,<  / / __/
 / /  / / /_/ / /_/ /_/ /  __/ |/ / /| |/ / /_  
/_/  /_/\__,_/\__/_____/\___/|___/_/ |_/_/\__/  
                                                 
"#.bright_blue());
    
    println!("{}", "Welcome to MacDevKit - Your Ultimate macOS Development Environment Setup Tool".yellow());
    println!();
    println!("{}", "This CLI tool will help you:".cyan());
    println!("  {}  Install essential developer tools", "✓".green());
    println!("  {}  Configure your development environment", "✓".green());
    println!("  {}  Set up programming languages and frameworks", "✓".green());
    println!("  {}  Install useful applications", "✓".green());
    println!("  {}  Optimize your macOS settings", "✓".green());
    println!();
}

fn run_interactive_menu() {
    let options = vec![
        "Full Setup",
        "Install Xcode Command Line Tools",
        "Install Homebrew",
        "Install and configure Git",
        "Generate SSH key",
        "Install Visual Studio Code",
        "Install Node.js via NVM",
        "Install iTerm2",
        "Install Oh My Zsh",
        "Install Docker",
        "Install additional developer tools",
        "Install useful applications",
        "Configure macOS settings",
        "Create development workspace",
        "Exit",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();
    
    match selection {
        0 => run_full_setup(),
        1 => install_xcode_tools(),
        2 => install_homebrew(),
        3 => install_git(),
        4 => generate_ssh_key(),
        5 => install_vscode(),
        6 => install_node(),
        7 => install_iterm(),
        8 => install_oh_my_zsh(),
        9 => install_docker(),
        10 => install_dev_tools(),
        11 => install_apps(),
        12 => configure_macos(),
        13 => create_workspace(),
        14 => println!("{}", "Goodbye!".green()),
        _ => unreachable!(),
    }
}

fn run_full_setup() {
    println!("{}", "\n==== Running Full Setup ====\n".blue());
    
    // Run all steps sequentially with confirmation for each
    if confirm_step("Install Xcode Command Line Tools") {
        install_xcode_tools();
    }
    
    if confirm_step("Install Homebrew") {
        install_homebrew();
    }
    
    if confirm_step("Install and Configure Git") {
        install_git();
    }
    
    if confirm_step("Generate SSH Key") {
        generate_ssh_key();
    }
    
    if confirm_step("Install Visual Studio Code") {
        install_vscode();
    }
    
    if confirm_step("Install Node.js via NVM") {
        install_node();
    }
    
    if confirm_step("Install iTerm2") {
        install_iterm();
    }
    
    if confirm_step("Install Oh My Zsh") {
        install_oh_my_zsh();
    }
    
    if confirm_step("Install Docker") {
        install_docker();
    }
    
    if confirm_step("Install Additional Developer Tools") {
        install_dev_tools();
    }
    
    if confirm_step("Install Useful Applications") {
        install_apps();
    }
    
    if confirm_step("Configure macOS Settings") {
        configure_macos();
    }
    
    if confirm_step("Create Development Workspace") {
        create_workspace();
    }
    
    println!("{}", "\n==== Setup Complete! ====\n".blue());
    println!("{}", "Your Mac has been set up for development.".green());
    println!("{}", "Some changes may require a restart to take effect.".yellow());
    println!("{}", "Enjoy your new development environment!".green());
    
    if confirm_restart() {
        restart_computer();
    }
}

fn confirm_step(step_name: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Do you want to {}?", step_name))
        .default(true)
        .interact()
        .unwrap_or(false)
}

fn confirm_restart() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to restart your computer now?")
        .default(false)
        .interact()
        .unwrap_or(false)
}

fn command_exists(command: &str) -> bool {
    which(command).is_ok()
}

// Implementation of step functions using ScriptHandler
fn install_xcode_tools() {
    let script_handler = ScriptHandler::new();
    match script_handler.run_section("xcode") {
        Ok(success) => {
            if success {
                println!("{}", "Xcode Command Line Tools installation completed".green());
            } else {
                println!("{}", "Xcode Command Line Tools installation failed".red());
            }
        },
        Err(e) => {
            println!("{}", format!("Error: {}", e).red());
            
            // Fallback to direct implementation
            if command_exists("xcode-select") {
                println!("{}", "✓ Xcode Command Line Tools already installed".green());
            } else {
                println!("{}", "Installing Xcode Command Line Tools...".cyan());
                let _ = Command::new("xcode-select")
                    .args(["--install"])
                    .status();
                println!("{}", "Xcode Command Line Tools installation triggered".green());
                println!("Please wait for the installation to complete.");
            }
        }
    }
}

fn install_homebrew() {
    let script_handler = ScriptHandler::new();
    match script_handler.run_section("brew") {
        Ok(success) => {
            if success {
                println!("{}", "Homebrew installation completed".green());
            } else {
                println!("{}", "Homebrew installation failed".red());
            }
        },
        Err(e) => {
            println!("{}", format!("Error: {}", e).red());
            
            // Fallback to direct implementation
            if command_exists("brew") {
                println!("{}", "✓ Homebrew already installed".green());
                
                let _ = Command::new("brew")
                    .arg("update")
                    .status();
                println!("{}", "Homebrew updated".green());
            } else {
                println!("{}", "Installing Homebrew...".cyan());
                let brew_install_script = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
                let _ = Command::new("bash")
                    .args(["-c", brew_install_script])
                    .status();
                println!("{}", "Homebrew installed".green());
            }
        }
    }
}

fn install_git() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("git") {
        println!("{}", format!("Error: {}", e).red());
        
        // Fallback implementation
        if command_exists("git") {
            println!("{}", "✓ Git already installed".green());
        } else {
            println!("{}", "Installing Git...".cyan());
            let _ = Command::new("brew")
                .args(["install", "git"])
                .status();
            println!("{}", "Git installed".green());
        }
        
        // Configure Git if not already
        let username_output = Command::new("git")
            .args(["config", "--global", "user.name"])
            .output();
        
        // 检查是否有输出，而不是使用 unwrap_or_else 直接创建 Output
        let has_username = username_output.is_ok() && 
            !username_output.as_ref().unwrap().stdout.is_empty();
        
        if !has_username {
            // Let user enter git configuration
            let git_username = dialoguer::Input::<String>::new()
                .with_prompt("Enter your Git username")
                .interact()
                .unwrap_or_else(|_| String::from(""));
                
            let _ = Command::new("git")
                .args(["config", "--global", "user.name", &git_username])
                .status();
                
            let git_email = dialoguer::Input::<String>::new()
                .with_prompt("Enter your Git email")
                .interact()
                .unwrap_or_else(|_| String::from(""));
                
            let _ = Command::new("git")
                .args(["config", "--global", "user.email", &git_email])
                .status();
                
            // Set some sensible Git defaults
            let _ = Command::new("git")
                .args(["config", "--global", "init.defaultBranch", "main"])
                .status();
                
            let _ = Command::new("git")
                .args(["config", "--global", "core.editor", "code --wait"])
                .status();
                
            let _ = Command::new("git")
                .args(["config", "--global", "pull.rebase", "false"])
                .status();
                
            println!("{}", "Git configured".green());
        } else {
            println!("{}", "✓ Git already configured".green());
        }
    }
}

// Implement the remaining functions using the ScriptHandler
fn generate_ssh_key() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("ssh") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn install_vscode() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("vscode") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn install_node() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("node") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn install_iterm() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("iterm") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn install_oh_my_zsh() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("zsh") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn install_docker() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("docker") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn install_dev_tools() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("devtools") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn install_apps() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("apps") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn configure_macos() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("macos") {
        println!("{}", format!("Error: {}", e).red());
        // Fallback implementation would go here
    }
}

fn create_workspace() {
    let script_handler = ScriptHandler::new();
    if let Err(e) = script_handler.run_section("workspace") {
        println!("{}", format!("Error: {}", e).red());
        
        // Fallback implementation
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
        let _ = Command::new("mkdir")
            .args(["-p", &format!("{}/Workspace", home_dir)])
            .status();
        println!("{}", "Created ~/Workspace directory".green());
    }
}

fn restart_computer() {
    println!("{}", "Restarting your computer now...".cyan());
    let _ = Command::new("sudo")
        .args(["shutdown", "-r", "now"])
        .status();
} 