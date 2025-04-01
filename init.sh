#!/bin/bash

# MacDevKit: 一个全面的 macOS 开发环境设置脚本
# 作者: jarvislin94
# 许可证: MIT

# 设置颜色以提高可读性
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # 无颜色

# 打印章节标题的函数
print_section() {
    echo -e "\n${BLUE}==== $1 ====${NC}\n"
}

# 打印成功消息的函数
print_success() {
    echo -e "${GREEN}✓ $1${NC}\n"
}

# 打印错误消息的函数
print_error() {
    echo -e "${RED}✗ $1${NC}\n"
}

# 打印信息消息的函数
print_info() {
    echo -e "${CYAN}ℹ $1${NC}\n"
}

# 检查命令是否存在的函数
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 重置Homebrew镜像源为官方的函数
reset_brew_mirror() {
    if [[ -n "$HOMEBREW_BREW_GIT_REMOTE" ]]; then
        unset HOMEBREW_BREW_GIT_REMOTE
        unset HOMEBREW_CORE_GIT_REMOTE
        unset HOMEBREW_BOTTLE_DOMAIN
        print_info "已重置为官方源"
    fi
}

# 请求确认的函数
confirm_step() {
    local step_name="$1"
    local step_description="$2"
    
    echo -e "\n${YELLOW}▶ 步骤: ${step_name}${NC}\n"
    echo -e "${CYAN}描述:${NC} ${step_description}\n"
    
    local response=""
    echo -e "${YELLOW}是否要继续此步骤? (y/n)${NC}"
    read -r response
    
    if [[ "$response" =~ ^[Yy] ]]; then
        return 0 # 真 - 继续步骤
    else
        echo -e "${YELLOW}跳过: ${step_name}${NC}"
        return 1 # 假 - 跳过步骤
    fi
}

# 欢迎信息
clear
cat << "EOF"
    __  ___          ____             __ __ _ __ 
   /  |/  /___ _____/ __ \___ _   __/ //_/(_) /_
  / /|_/ / __ `/ __/ / / / _ \ | / / ,<  / / __/
 / /  / / /_/ / /_/ /_/ /  __/ |/ / /| |/ / /_  
/_/  /_/\__,_/\__/_____/\___/|___/_/ |_/_/\__/  
                                                 
EOF

echo -e "${YELLOW}欢迎使用 MacDevKit - 您的终极 macOS 开发环境设置工具${NC}"
echo
echo -e "${CYAN}本脚本将帮助您:${NC}"
echo -e "  ${GREEN}✓${NC} 安装必要的开发者工具"
echo -e "  ${GREEN}✓${NC} 配置您的开发环境"
echo -e "  ${GREEN}✓${NC} 设置编程语言和框架"
echo -e "  ${GREEN}✓${NC} 安装有用的应用程序"
echo -e "  ${GREEN}✓${NC} 优化您的 macOS 设置"
echo
echo -e "${YELLOW}每个步骤都会有说明，您可以选择继续或跳过。${NC}"
echo -e "${RED}注意: 某些操作可能需要输入您的密码。${NC}"
echo
echo -e "${CYAN}按 Enter 开始设置过程，或按 Ctrl+C 退出...${NC}"
read -r

# 步骤 1: 安装 Xcode 命令行工具
if confirm_step "安装 Xcode 命令行工具" "Xcode 命令行工具是 macOS 上许多开发任务所必需的。这包括编译器、构建工具、Git 等。这些工具对大多数开发工作至关重要，也是 Homebrew 的必备组件。"; then
    print_section "正在安装 Xcode 命令行工具"
    if command_exists xcode-select; then
        print_success "Xcode 命令行工具已安装"
    else
        xcode-select --install
        print_success "已触发 Xcode 命令行工具安装"
        echo "请等待安装完成后再继续。"
        echo "安装完成后按 Enter 键继续。"
        read -r
    fi
fi

# 步骤 2: 安装 Homebrew
if confirm_step "安装 Homebrew" "Homebrew 是 macOS 的包管理器，可让您轻松安装软件和开发工具。它是本脚本中大多数工具的安装基础。"; then
    print_section "正在安装 Homebrew"

    # 询问是否使用国内镜像源
    echo -e "${YELLOW}是否要使用国内镜像源加速安装? (y/n)${NC}"
    read -r use_mirror
    if [[ "$use_mirror" =~ ^[Yy] ]]; then
        print_info "正在配置国内镜像源"

        # 选择镜像源
        echo -e "${CYAN}请选择镜像源:${NC}"
        echo "1) 清华大学镜像源"
        echo "2) 中国科学技术大学镜像源"
        read -r mirror_choice

        case $mirror_choice in
            1)
                # 清华源
                export HOMEBREW_BREW_GIT_REMOTE="https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git"
                export HOMEBREW_CORE_GIT_REMOTE="https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git"
                export HOMEBREW_BOTTLE_DOMAIN="https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles"
                ;;
            2)
                # 中科大源
                export HOMEBREW_BREW_GIT_REMOTE="https://mirrors.ustc.edu.cn/brew.git"
                export HOMEBREW_CORE_GIT_REMOTE="https://mirrors.ustc.edu.cn/homebrew-core.git"
                export HOMEBREW_BOTTLE_DOMAIN="https://mirrors.ustc.edu.cn/homebrew-bottles"
                ;;
            *)
                print_error "无效选择，将使用官方源"
                ;;
        esac

        print_success "已配置镜像源"
    fi
    if command_exists brew; then
        print_success "Homebrew 已安装"

        # 重置镜像源为官方源(如果使用了镜像源)
        reset_brew_mirror

        brew update
        print_success "Homebrew 已更新"
    else
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        print_success "Homebrew 已安装"
        
        # 重置镜像源为官方源(如果使用了镜像源)
        reset_brew_mirror

        # 为 Intel 和 Apple Silicon Mac 添加 Homebrew 到 PATH
        if [[ $(uname -m) == 'arm64' ]]; then
            echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
            eval "$(/opt/homebrew/bin/brew shellenv)"
            print_success "已为 Apple Silicon Mac 添加 Homebrew 到 PATH"
        else
            echo 'eval "$(/usr/local/bin/brew shellenv)"' >> ~/.zprofile
            eval "$(/usr/local/bin/brew shellenv)"
            print_success "已为 Intel Mac 添加 Homebrew 到 PATH"
        fi
    fi
fi

# 步骤 3: 安装并配置 Git
if confirm_step "安装并配置 Git" "Git 是用于跟踪源代码变更的版本控制系统。此步骤将安装 Git 并使用您的姓名和电子邮件设置全局 Git 配置。"; then
    print_section "正在安装并配置 Git"
    if command_exists git; then
        print_success "Git 已安装"
    else
        brew install git
        print_success "Git 已安装"
    fi

    # 如果尚未配置 Git 则进行配置
    if [ -z "$(git config --global user.name)" ]; then
        echo "输入您的 Git 用户名:"
        read -r git_username
        git config --global user.name "$git_username"
        
        echo "输入您的 Git 邮箱:"
        read -r git_email
        git config --global user.email "$git_email"
        
        # 设置一些合理的 Git 默认值
        git config --global init.defaultBranch main
        git config --global core.editor "code --wait"
        git config --global pull.rebase false
        
        print_success "Git 已配置"
    else
        print_success "Git 已配置"
    fi
fi

# 步骤 4: 生成 SSH 密钥
if confirm_step "生成 SSH 密钥" "SSH 密钥用于与 GitHub、GitLab 和远程服务器等服务的安全认证。此步骤将生成 Ed25519 SSH 密钥对并将其添加到您的 SSH 代理。"; then
    print_section "正在生成 SSH 密钥"
    if [ -f ~/.ssh/id_ed25519 ]; then
        print_success "SSH 密钥已存在"
    else
        echo "正在生成新的 SSH 密钥 (Ed25519 算法)"
        ssh-keygen -t ed25519 -C "$(git config --global user.email)"
        
        # 在后台启动 ssh-agent
        eval "$(ssh-agent -s)"
        
        # 将 SSH 密钥添加到 ssh-agent
        ssh-add ~/.ssh/id_ed25519
        
        # 将 SSH 密钥复制到剪贴板
        if command_exists pbcopy; then
            pbcopy < ~/.ssh/id_ed25519.pub
            print_success "SSH 公钥已复制到剪贴板"
            echo "请将此密钥添加到您的 GitHub/GitLab 账户"
            echo "公钥: $(cat ~/.ssh/id_ed25519.pub)"
        else
            echo "公钥: $(cat ~/.ssh/id_ed25519.pub)"
            echo "请复制此密钥并添加到您的 GitHub/GitLab 账户"
        fi
    fi
fi

# 步骤 5: 安装 Visual Studio Code
if confirm_step "安装 Visual Studio Code" "Visual Studio Code 是一款流行的代码编辑器，具有语法高亮、智能代码补全和调试支持等功能。它拥有丰富的扩展生态系统，支持各种编程语言和工具。"; then
    print_section "正在安装 Visual Studio Code"
    if command_exists code; then
        print_success "VS Code 已安装"
    else
        brew install --cask visual-studio-code
        print_success "VS Code 已安装"
    fi
fi

# 安装 VS Code 扩展
if command_exists code; then
    if confirm_step "安装 VS Code 扩展" "此步骤将安装有用的 VS Code 扩展，包括对 TypeScript、ESLint、Prettier、Python、Docker、GitHub Copilot、GitLens、Remote Containers、Live Server 和 Code Spell Checker 的支持。"; then
        print_section "正在安装 VS Code 扩展"
        extensions=(
            "EditorConfig.EditorConfig"                 # 编辑器配置标准化
            "ms-vscode.vscode-typescript-next"        # TypeScript 支持
            "dbaeumer.vscode-eslint"                  # JavaScript/TypeScript 代码检查
            "esbenp.prettier-vscode"                  # 代码格式化工具
            "github.copilot"                          # GitHub AI 编程助手
            "eamodio.gitlens"                         # Git 历史查看和代码作者追踪
            "streetsidesoftware.code-spell-checker"   # 代码拼写检查
            "antfu.goto-alias"                        # 快速跳转到别名定义
            "donjayamanne.githistory"                 # Git 历史可视化
            "VisualStudioExptTeam.vscodeintellicode"  # AI 辅助代码补全
            "PKief.material-icon-theme"               # 文件图标主题
            "christian-kohler.path-intellisense"      # 路径自动补全
            "Vue.volar"                               # Vue 3 开发支持
            "alefragnani.project-manager"             # 项目管理工具
            "Gruntfuggly.todo-tree"                   # TODO 注释管理
            "shd101wyy.markdown-preview-enhanced"     # Markdown 预览增强
            "saoudrizwan.claude-dev"                  # Cline AI 开发助手
        )
        
        for extension in "${extensions[@]}"; do
            print_info "正在安装扩展: $extension"
            code --install-extension "$extension"
        done
        print_success "VS Code 扩展已安装"
    fi
fi

# 步骤 6: 通过 NVM 安装 Node.js
if confirm_step "通过 NVM 安装 Node.js" "Node 版本管理器 (NVM) 允许您安装和管理多个版本的 Node.js。此步骤将安装 NVM、最新的 LTS 版本的 Node.js 以及几个有用的全局 npm 包，如 yarn、typescript 和 nodemon。"; then
    print_section "正在通过 NVM 安装 Node.js"
    if command_exists nvm; then
        print_success "NVM 已安装"
    else
        brew install nvm
        
        # 创建 NVM 目录
        mkdir -p ~/.nvm
        
        # 将 NVM 添加到 shell 配置文件
        echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.zshrc
        echo '[ -s "/opt/homebrew/opt/nvm/nvm.sh" ] && \. "/opt/homebrew/opt/nvm/nvm.sh"' >> ~/.zshrc
        echo '[ -s "/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm" ] && \. "/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm"' >> ~/.zshrc
        
        # 加载 NVM
        export NVM_DIR="$HOME/.nvm"
        [ -s "/opt/homebrew/opt/nvm/nvm.sh" ] && \. "/opt/homebrew/opt/nvm/nvm.sh"
        
        print_success "NVM 已安装"
        
        # 安装最新的 LTS 版本的 Node.js
        print_info "正在安装最新的 LTS 版本的 Node.js"
        nvm install --lts
        nvm use --lts
        nvm alias default node
        
        print_success "Node.js LTS 已安装并设为默认"
        
        # 安装全局 npm 包
        if confirm_step "安装全局 npm 包" "这将全局安装有用的 npm 包: yarn/pnpm (包管理器)、Typescript、ts-node (TypeScript 执行)、nodemon (Node.js 自动重启)、http-server (简单 HTTP 服务器)、eslint (代码检查器) 和 prettier (代码格式化工具)。"; then
            npm_packages=(
                "yarn"
                "pnpm"
                "typescript"
                "ts-node"
                "nodemon"
                "http-server"
                "eslint"
                "prettier"
            )
            
            for package in "${npm_packages[@]}"; do
                print_info "正在安装 npm 包: $package"
                npm install -g "$package"
            done
            
            print_success "全局 npm 包已安装"
        fi
    fi
fi

# 步骤 7: 安装 iTerm2
if confirm_step "安装 iTerm2" "iTerm2 是 macOS 默认终端应用的替代品。它具有许多附加功能，如分屏、搜索、自动完成和更多自定义选项。"; then
    print_section "正在安装 iTerm2"
    if [ -d "/Applications/iTerm.app" ]; then
        print_success "iTerm2 已安装"
    else
        brew install --cask iterm2
        print_success "iTerm2 已安装"
    fi
fi

# 步骤 8: 安装并配置 Oh My Zsh
if confirm_step "安装 Oh My Zsh" "Oh My Zsh 是管理 Zsh 配置的框架。它包含有用的函数、插件和主题。此步骤还将安装 Powerlevel10k 主题以及用于自动建议和语法高亮的实用插件。"; then
    print_section "正在安装 Oh My Zsh"
    if [ -d "$HOME/.oh-my-zsh" ]; then
        print_success "Oh My Zsh 已安装"
    else
        sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
        
        # 安装 Powerlevel10k 主题
        print_info "正在安装 Powerlevel10k 主题"
        git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
        
        # 安装有用的插件
        print_info "正在安装 zsh-autosuggestions 插件"
        git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
        
        print_info "正在安装 zsh-syntax-highlighting 插件"
        git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
        
        # 更新 .zshrc
        print_info "正在更新 .zshrc 配置"
        sed -i '' 's/ZSH_THEME="robbyrussell"/ZSH_THEME="powerlevel10k\/powerlevel10k"/' ~/.zshrc
        sed -i '' 's/plugins=(git)/plugins=(git zsh-autosuggestions zsh-syntax-highlighting)/' ~/.zshrc
        
        print_success "Oh My Zsh 已安装并配置"
    fi
fi

# 步骤 9: 安装 Docker
if confirm_step "安装 Docker" "Docker 是一个用于在容器中开发、运输和运行应用程序的平台。容器允许您将应用程序与其所有依赖项打包并在任何环境中运行。"; then
    print_section "正在安装 Docker"
    if command_exists docker; then
        print_success "Docker 已安装"
    else
        brew install --cask docker
        print_success "Docker 已安装"
        print_info "请打开 Docker.app 以完成安装"
    fi
fi

# 步骤 10: 安装其他开发者工具
if confirm_step "安装其他开发者工具" "此步骤将安装各种开发工具，包括: 编程语言 (Python, Go, Rust)、数据库工具 (MySQL, SQLite) "; then
    print_section "正在安装其他开发者工具"

    # 编程语言
    if confirm_step "安装编程语言" "这将安装 Python、Go 和 Rust 编程语言。"; then
        print_info "正在安装编程语言 (Python, Go, Rust)"
        brew install python go rust
    fi

    # 数据库工具
    if confirm_step "安装数据库工具" "这将安装 MySQL 和 SQLite 数据库系统。"; then
        print_info "正在安装数据库工具 (MySQL, SQLite)"
        brew install mysql sqlite
    fi

    # 有用的命令行工具
    if confirm_step "安装命令行工具" "这将安装各种有用的命令行工具，如 jq (JSON 处理器)、ripgrep (快速 grep)、bat (更好的 cat) 等。"; then
        print_info "正在安装命令行工具"
        cli_tools=(
            "jq"           # JSON 处理器
            "ripgrep"      # 快速 grep
            "fd"           # 快速查找
            "bat"          # 更好的 cat
            "exa"          # 更好的 ls
            "htop"         # 更好的 top
            "tldr"         # 简化的 man 页面
            "fzf"          # 模糊查找器
            "tmux"         # 终端复用器
            "tree"         # 目录树
            "wget"         # 文件下载器
            "httpie"       # HTTP 客户端
            "gh"           # GitHub CLI
            "cloc"         # 代码行数统计
            "z.lua"        # 路径自动补全
            "shellcheck"   # shell 脚本语法检查
        )

        for tool in "${cli_tools[@]}"; do
            print_info "正在安装: $tool"
            brew install "$tool"

        done
    fi

    # QuickLook 扩展套件
    if confirm_step "安装 QuickLook 扩展套件" "这将安装增强文件预览功能的 QuickLook 插件，支持代码高亮、Markdown预览等。"; then
        print_info "正在安装 QuickLook 扩展套件"
        brew install --cask qlcolorcode qlmarkdown quicklook-json qlvideo suspicious-package
        print_success "QuickLook 扩展套件已安装"
    fi

    # 开发工具
    if confirm_step "安装开发工具" "这将安装Charles抓包工具和H3开发者工具箱。"; then
        print_info "正在安装开发工具"
        brew install --cask charles h3
        print_success "开发工具已安装"
    fi

    # IDE 工具
    if confirm_step "安装IDE工具" "这将安装微信开发者工具和Cursor AI编辑器。"; then
        print_info "正在安装IDE工具"
        brew install --cask wechatwebdevtools cursor
        print_success "IDE工具已安装"
    fi

    print_success "已安装其他开发者工具"
fi

# Step 11: 通过 Homebrew Cask 安装有用的应用程序
print_section "正在安装有用的应用程序"

apps=(
    # 浏览器
    "google-chrome"
    "firefox"
    
    # 生产力工具
    "rectangle"     # 窗口管理器
    "alfred"        # Spotlight 替代品
    "alt-tab"       # 窗口切换工具
    "cheatsheet"    # 快捷键查看工具
    
    # 笔记/文档工具
    "notion"        # 笔记工具
    "obsidian"      # 笔记工具
    "yuque"         # 语雀文档工具
    
    # 设计工具
    "figma"          # 设计工具
    
    # 开发工具
    "dash"          # API文档查看器
    "apifox"        # API调试工具
    "switchhosts"   # Hosts管理工具
    
    # 媒体工具
    "iina"           # 视频播放器
    "the-unarchiver" # 解压工具
    "skim"           # PDF阅读器
    "squoosh"        # 图片压缩工具
    
    # 文件传输
    "localsend"      # 局域网文件传输
    
    # 系统工具
    "tencent-lemon"  # 系统清理工具
    
    # 通讯工具
    "wechat"          # 微信
    "qq"              # QQ
    "telegram"        # Telegram
    "dingtalk"        # 钉钉
    "wechatwork"      # 企业微信
    "feishu"          # 飞书
    "tencent-meeting" # 腾讯会议
)

echo -e "${CYAN}以下应用程序可供安装:${NC}"
for i in "${!apps[@]}"; do
    echo -e "  ${YELLOW}$((i+1)).${NC} ${apps[$i]}"
done

if confirm_step "安装应用程序" "这将安装各类应用程序，包括浏览器、生产力工具、笔记/文档工具、设计工具、开发工具、媒体工具、文件传输工具、系统工具和通讯工具等。"; then
    for app in "${apps[@]}"; do
        if confirm_step "安装 $app" "这将在您的系统上安装 $app。"; then
            print_info "正在安装: $app"
            brew install --cask "$app"
            print_success "$app 已安装"
        fi
    done
    print_success "应用程序安装完成"
else
    echo "跳过应用程序安装"
fi

# Step 12: 配置 macOS 设置
if confirm_step "配置 macOS 设置" "这将配置各种针对开发优化的 macOS 设置，包括 Finder 偏好设置、键盘设置和安全选项。这些更改将使您的 Mac 更加适合开发。"; then
    print_section "正在配置 macOS 设置"

    # 在 Finder 中显示隐藏文件
    if confirm_step "在 Finder 中显示隐藏文件" "这将使 Finder 显示隐藏文件(以点开头的文件)。"; then
        defaults write com.apple.finder AppleShowAllFiles -bool true
        print_success "Finder 已设置为显示隐藏文件"
    fi

    # 在 Finder 中显示路径栏
    if confirm_step "在 Finder 中显示路径栏" "这将在 Finder 窗口底部显示路径栏。"; then
        defaults write com.apple.finder ShowPathbar -bool true
        print_success "Finder 路径栏已启用"
    fi

    # 在 Finder 中显示状态栏
    if confirm_step "在 Finder 中显示状态栏" "这将在 Finder 窗口底部显示状态栏。"; then
        defaults write com.apple.finder ShowStatusBar -bool true
        print_success "Finder 状态栏已启用"
    fi

    # 禁用按键长按功能，启用按键重复
    if confirm_step "启用按键重复" "这将禁用按键长按功能并启用按键重复，这对编码很有用。"; then
        defaults write NSGlobalDomain ApplePressAndHoldEnabled -bool false
        print_success "按键重复已启用"
    fi

    # 设置更快的键盘重复速率
    if confirm_step "设置更快的键盘重复速率" "这将使按键在被按住时重复得更快。"; then
        defaults write NSGlobalDomain KeyRepeat -int 2
        defaults write NSGlobalDomain InitialKeyRepeat -int 15
        print_success "键盘重复速率已提高"
    fi

    # 禁用自动更正
    if confirm_step "禁用自动更正" "这将禁用自动文本更正功能，这在编码时可能会很烦人。"; then
        defaults write NSGlobalDomain NSAutomaticSpellingCorrectionEnabled -bool false
        print_success "自动更正已禁用"
    fi

    # 睡眠或屏幕保护程序启动后立即要求输入密码
    if confirm_step "增强安全设置" "这将在睡眠或屏幕保护程序启动后立即要求输入密码。"; then
        defaults write com.apple.screensaver askForPassword -int 1
        defaults write com.apple.screensaver askForPasswordDelay -int 0
        print_success "安全设置已增强"
    fi

    # 将屏幕截图保存到桌面
    if confirm_step "配置屏幕截图位置" "这将把屏幕截图保存到桌面上的'Screenshots'文件夹。"; then
        mkdir -p "${HOME}/Desktop/Screenshots"
        defaults write com.apple.screencapture location -string "${HOME}/Desktop/Screenshots"
        print_success "屏幕截图位置已设置为 Desktop/Screenshots"
    fi

    # 以 PNG 格式保存屏幕截图
    if confirm_step "设置屏幕截图格式为 PNG" "这将以 PNG 格式保存屏幕截图。"; then
        defaults write com.apple.screencapture type -string "png"
        print_success "屏幕截图格式已设置为 PNG"
    fi

    # 重启受影响的应用程序
    print_info "正在重启 Finder 和 SystemUIServer 以应用更改"
    killall Finder
    killall SystemUIServer

    print_success "macOS 设置已配置"
else
    echo "跳过 macOS 配置"
fi

# Step 13: 创建开发工作区
if confirm_step "创建开发工作区" "这将在您的主文件夹中创建一个'Code'目录，用于组织您的开发项目。"; then
    print_section "正在创建开发工作区"
    mkdir -p ~/Code
    print_success "已创建 ~/Code 目录"
fi

# 最终消息
print_section "设置完成!"
echo -e "${GREEN}您的 Mac 已设置为开发环境。${NC}"
echo -e "${YELLOW}某些更改可能需要重启才能生效。${NC}"
echo -e "${GREEN}享受您的新开发环境!${NC}"

# 建议重启
if confirm_step "重启计算机" "建议重启计算机以确保所有更改正确生效。"; then
    print_info "正在重启您的计算机..."
    sudo shutdown -r now
fi
