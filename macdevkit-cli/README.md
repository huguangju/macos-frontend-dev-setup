# MacDevKit CLI

A Rust CLI wrapper for the MacDevKit setup script. This tool provides a modern command-line interface for setting up your macOS development environment.

## Overview

MacDevKit CLI is a Rust-based command-line tool that wraps the functionality of the MacDevKit bash script (`init.sh`). It provides a more user-friendly and interactive experience while leveraging the proven functionality of the original script.

## Features

- Modern CLI interface with subcommands
- Interactive menu for easy navigation
- Colorful output for better readability
- Progress indicators and confirmations
- Works seamlessly with the original MacDevKit bash script

## Installation

### Prerequisites

- Rust and Cargo (Install via [rustup](https://rustup.rs/))
- macOS (This tool is specifically designed for macOS)

### Building from Source

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/macdevkit-cli.git
   cd macdevkit-cli
   ```

2. Build the project:

   ```
   cargo build --release
   ```

3. Install the binary (optional):
   ```
   cargo install --path .
   ```

## Usage

### Interactive Mode

Simply run the tool without any arguments to enter interactive mode:

```
macdevkit-cli
```

This will display a menu where you can select the operation you want to perform.

### Using Subcommands

You can also use specific subcommands to run individual setup steps:

```
USAGE:
    macdevkit-cli [SUBCOMMAND]

SUBCOMMANDS:
    setup       Run the full setup with interactive prompts
    xcode       Install Xcode Command Line Tools
    brew        Install Homebrew
    git         Install and configure Git
    ssh         Generate SSH key
    vscode      Install Visual Studio Code
    node        Install Node.js via NVM
    iterm       Install iTerm2
    zsh         Install Oh My Zsh
    docker      Install Docker
    devtools    Install additional developer tools
    apps        Install useful applications
    macos       Configure macOS settings
    workspace   Create development workspace
    help        Print this message or the help of the given subcommand(s)
```

### Examples

Install Homebrew:

```
macdevkit-cli brew
```

Set up VS Code:

```
macdevkit-cli vscode
```

Run the full setup:

```
macdevkit-cli setup
```

## Dependencies

This project relies on:

- The original MacDevKit bash script (`init.sh`)
- Several Rust crates:
  - clap - Command line argument parsing
  - colored - Terminal text coloring
  - dialoguer - Interactive user prompts
  - indicatif - Progress indicators
  - which - Command existence checking

## License

MIT License - Same as the original MacDevKit project.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Related Projects

- [MacDevKit](https://github.com/jarvislin94/MacDevKit) - The original bash script version
