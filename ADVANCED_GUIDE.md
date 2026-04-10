# chess-tui 完整指南

## 目录

1. [简介](#简介)
2. [安装](#安装)
3. [快速入门](#快速入门)
4. [基础用法](#基础用法)
5. [高级功能](#高级功能)
6. [在线对弈](#在线对弈)
7. [引擎配置](#引擎配置)
8. [自定义与主题](#自定义与主题)
9. [PGN 管理](#pgn 管理)
10. [故障排除](#故障排除)
11. [完整示例](#完整示例)
12. [性能基准](#性能基准)
13. [常见问题](#常见问题)

---

## 简介

**chess-tui** 是一个免费的开源终端国际象棋游戏，使用 Rust 编写。支持以下功能：

- ♟️ **本地对弈** - 与朋友在同一台设备上对战
- 🤖 **AI 对战** - 与任何 UCI 引擎对战 (Stockfish、Leela 等)
- 🌐 **在线对弈** - 通过 Lichess 与全球玩家对战
- 📊 **棋局分析** - 使用引擎分析你的棋局
- 🎨 **主题自定义** - 多种棋盘主题可选
- 📝 **PGN 导入导出** - 保存和分析棋局

**平台支持**: macOS、Linux、Windows

### 版本信息

| 版本 | 发布日期 | 主要更新 |
|------|---------|---------|
| v0.28.x | 2026-04 | 当前稳定版 |
| v0.27.x | 2026-03 | Lichess 改进 |
| v0.26.x | 2026-02 | 主题系统重构 |

### 系统要求

| 平台 | 最低要求 | 推荐配置 |
|------|---------|---------|
| **Linux** | 2GB RAM, 任何终端 | 4GB RAM, 真彩色终端 |
| **macOS** | 10.15+ | 11.0+ |
| **Windows** | 10+ | 11 + Windows Terminal |

---

## 安装

### Homebrew (macOS/Linux)

```bash
# 添加仓库
brew tap thomas-mauran/tap

# 安装
brew install chess-tui

# 验证安装
chess-tui --version

# 升级
brew upgrade chess-tui

# 卸载
brew uninstall chess-tui
```

### Cargo (所有平台)

```bash
# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 chess-tui
cargo install chess-tui

# 验证
chess-tui --version

# 卸载
cargo uninstall chess-tui

# 重新安装 (清理缓存)
cargo install --force chess-tui
```

### Debian/Ubuntu

```bash
# 下载最新 .deb 包
DEB_URL=$(curl -s "https://api.github.com/repos/thomas-mauran/chess-tui/releases/latest" | jq -r '.assets[] | select(.name | endswith(".deb")) | .browser_download_url')

# 下载并安装
curl -LO "$DEB_URL"
sudo dpkg -i "$(basename "$DEB_URL")"
sudo apt-get install -f

# 验证
chess-tui --version

# 卸载
sudo apt remove chess-tui
```

### Arch Linux

```bash
# 使用 AUR 助手
yay -S chess-tui
# 或
paru -S chess-tui

# 验证
chess-tui --version

# 升级
yay -S chess-tui
```

### Windows

#### 使用 Chocolatey

```powershell
# 安装 Chocolatey (如果未安装)
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# 安装 chess-tui
choco install chess-tui

# 验证
chess-tui --version

# 升级
choco upgrade chess-tui
```

#### 使用 Scoop

```powershell
# 安装 Scoop
iwr -useb get.scoop.sh | iex

# 安装 chess-tui
scoop install chess-tui

# 验证
chess-tui --version
```

#### 手动安装

1. 访问 [GitHub Releases](https://github.com/thomas-mauran/chess-tui/releases/latest)
2. 下载 `chess-tui-x86_64-pc-windows-msvc.zip`
3. 解压到任意目录 (如 `C:\Program Files\chess-tui\`)
4. 将目录添加到 PATH 环境变量
5. 打开命令提示符运行 `chess-tui`

```powershell
# 添加到用户 PATH (PowerShell)
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
[Environment]::SetEnvironmentVariable("Path", "$userPath;C:\Program Files\chess-tui", "User")
```

### Nix/NixOS

```nix
# configuration.nix
environment.systemPackages = [
  (pkgs.rustPlatform.buildRustPackage {
    pname = "chess-tui";
    version = "0.28.0";
    src = pkgs.fetchFromGitHub {
      owner = "thomas-mauran";
      repo = "chess-tui";
      rev = "v0.28.0";
      sha256 = "sha256-xxxxx";
    };
    cargoSha256 = "sha256-xxxxx";
  })
];
```

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/thomas-mauran/chess-tui
cd chess-tui

# 编译 Release 版本
cargo build --release

# 二进制位置
./target/release/chess-tui

# (可选) 安装到系统
sudo cp target/release/chess-tui /usr/local/bin/

# 运行测试
cargo test

# 编译特定功能
cargo build --release --features=lichess
```

**编译要求**:
- Rust 1.70+
- 2GB+ 可用内存
- 约 10 分钟编译时间
- 网络连接 (下载依赖)

---

## 快速入门

### 启动 chess-tui

```bash
chess-tui
```

### 界面导航

```
┌─────────────────────────────────────────────────────────┐
│                   chess-tui v0.28.0                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│     ╔═══════════════════════════════════════╗          │
│     ║  🎮 New Game                          ║          │
│     ║  🤖 vs Computer                       ║          │
│     ║  🌐 vs Lichess                        ║          │
│     ║  📂 Load PGN                          ║          │
│     ║  ⚙️  Settings                          ║          │
│     ║  ❌ Quit                               ║          │
│     ╚═══════════════════════════════════════╝          │
│                                                         │
│  ↑/↓: Navigate  Enter: Select  q: Quit                 │
└─────────────────────────────────────────────────────────┘
```

### 第一步：本地对弈

1. 启动 `chess-tui`
2. 选择 `🎮 New Game`
3. 白方先行，点击棋子选择
4. 移动到目标格子
5. 黑方回合重复步骤

### 基本操作

| 按键 | 功能 |
|------|------|
| `↑` `↓` `←` `→` | 移动光标 |
| `Enter` | 选择棋子/确认移动 |
| `Esc` | 取消选择/返回 |
| `u` | 撤销一步 |
| `r` | 重新开始 |
| `q` | 退出游戏 |
| `?` | 显示帮助 |
| `f` | 翻转棋盘 |
| `t` | 切换主题 |

### 棋子移动规则

**标准移动**:
- 选择棋子 (Enter)
- 移动到目标格 (Enter)
- 合法移动会高亮显示

**特殊移动**:

| 移动 | 操作 |
|------|------|
| 王车易位 | 移动王两格，车自动跳过 |
| 吃过路兵 | 按标准方式移动兵 |
| 兵升变 | 移动到最后一行，选择升变棋子 |

---

## 基础用法

### 游戏模式

#### 1. 本地双人对弈

```bash
chess-tui
# 选择 "New Game"
```

**适合**: 朋友面对面下棋、自我练习

**规则**:
- 标准国际象棋规则
- 无时间限制 (可自定义)
- 自动检测将死/和棋

#### 2. 人机对战

```bash
chess-tui
# 选择 "vs Computer"
# 选择引擎难度 (1-10)
```

**适合**: 单人练习、分析棋路

**难度级别**:
| 级别 | 描述 | Elo 近似 |
|------|------|---------|
| 1-2 | 初学者 | 400-800 |
| 3-4 | 休闲玩家 | 800-1200 |
| 5-6 | 俱乐部玩家 | 1200-1600 |
| 7-8 | 高级玩家 | 1600-2000 |
| 9-10 | 大师级别 | 2000+ |

#### 3. 在线对弈

```bash
chess-tui
# 选择 "vs Lichess"
# 登录 Lichess (首次需要)
# 创建或加入挑战
```

**适合**: 与真实玩家对战

**要求**:
- Lichess 账号 (免费)
- 网络连接
- OAuth token (自动获取)

#### 4. PGN 分析

```bash
# 加载 PGN 文件
chess-tui --load game.pgn

# 或使用菜单
chess-tui
# 选择 "Load PGN"
```

**适合**: 复盘分析、学习大师棋局

**支持格式**:
- `.pgn` - 标准 PGN 格式
- `.fen` - FEN 位置格式

---

## 高级功能

### UCI 引擎集成

#### 安装 Stockfish

```bash
# Ubuntu/Debian
sudo apt install stockfish

# macOS
brew install stockfish

# Windows
# 从 https://stockfishchess.org/download/ 下载
# 解压到 C:\Program Files\Stockfish\

# 验证安装
stockfish --version
```

#### 配置引擎

```toml
# ~/.config/chess-tui/config.toml
[engine]
path = "/usr/bin/stockfish"  # Linux
# path = "/usr/local/bin/stockfish"  # macOS
# path = "C:\\Program Files\\Stockfish\\stockfish.exe"  # Windows

depth = 15  # 思考深度 (1-22)
ponder = true  # 提前思考
multi_pv = 1  # 显示多个变化
```

#### 引擎难度设置

| 难度 | Depth | Elo 近似 | 说明 |
|------|-------|---------|------|
| 入门 | 1-5 | 400-800 | 新手级别 |
| 简单 | 6-10 | 800-1200 | 休闲玩家 |
| 中等 | 11-15 | 1200-1600 | 俱乐部玩家 |
| 困难 | 16-19 | 1600-2000 | 大师级别 |
| 专家 | 20-22 | 2000+ | 超人类水平 |

#### 多引擎支持

```toml
# ~/.config/chess-tui/engines.toml
[[engines]]
name = "Stockfish"
path = "/usr/bin/stockfish"
default_depth = 15

[[engines]]
name = "Leela"
path = "/usr/local/bin/lc0"
default_depth = 20

[[engines]]
name = "Komodo"
path = "/opt/komodo/komodo"
default_depth = 18
```

### 在线对弈 (Lichess)

#### OAuth 设置

1. 访问 https://lichess.org/account/oauth/token
2. 创建新 token
3. 勾选权限:
   - ✅ Play games with the bot API
   - ✅ Read user data
   - ✅ Write to the chat (可选)
4. 保存 token

#### 配置 Lichess

```toml
# ~/.config/chess-tui/lichess.toml
[lichess]
token = "your_token_here"
```

#### 创建挑战

```bash
chess-tui
# 选择 "vs Lichess"
# 选择 "Create Challenge"
# 设置参数:
#   - 时间控制 (Bullet, Blitz, Rapid, Classical)
#   - 颜色偏好 (White, Black, Random)
#   - 评级范围 (可选)
# 分享链接给对手
```

**时间控制选项**:

| 类型 | 时间 | 增量 |
|------|------|------|
| Bullet | 1-2 分钟 | 0-1 秒 |
| Blitz | 3-5 分钟 | 0-3 秒 |
| Rapid | 10-15 分钟 | 5-10 秒 |
| Classical | 30+ 分钟 | 30+ 秒 |

#### 加入挑战

```bash
# 使用挑战链接
chess-tui --lichess-challenge <challenge_id>

# 或在菜单中选择 "Join Challenge"
# 粘贴挑战 URL
```

#### 观看模式

```bash
# 观看 Lichess 直播比赛
chess-tui --watch <game_id>

# 或在菜单中选择 "Watch Games"
```

### PGN 管理

#### 导出棋局

```bash
# 游戏结束后自动保存到
# ~/.local/share/chess-tui/games/

# 或手动导出
chess-tui --export my-game.pgn

# 导出所有历史棋局
chess-tui --export-all games/
```

#### 导入分析

```bash
# 加载 PGN
chess-tui --load master-game.pgn

# 逐回合分析
# 使用引擎评估每一步
# 查看最佳着法建议
```

#### PGN 批量分析

```bash
#!/bin/bash
# analyze-tournament.sh

PGN_DIR="tournament_games"
OUTPUT_DIR="analysis_results"

mkdir -p "$OUTPUT_DIR"

for pgn in "$PGN_DIR"/*.pgn; do
    echo "Analyzing $(basename $pgn)..."
    chess-tui --load "$pgn" --analyze --depth 15 \
        --output "$OUTPUT_DIR/$(basename $pgn .pgn)_analysis.txt"
done

echo "Analysis complete! Results in $OUTPUT_DIR"
```

#### PGN 数据库

```bash
# 导出为数据库格式
chess-tui --export-db chess.db

# 查询数据库
chess-tui --query "opening: Sicilian Defense"
chess-tui --query "result: win"
chess-tui --query "elo: 2000+"
```

### 棋局分析

#### 实时评估

```toml
# ~/.config/chess-tui/config.toml
[analysis]
enabled = true
engine = "stockfish"
depth = 12
show_eval = true  # 显示评估分数
show_best_move = true  # 显示最佳着法
show_pv = true  # 显示主要变化
```

#### 评估显示格式

```
White: +0.45 (Stockfish depth 12)
Best move: Nf3
PV: Nf3 e5 Nxe5 d6

White: -1.23 (Stockfish depth 12)
Best move: Qd5
PV: Qd5 Nf6 Qxd6 exd6
```

#### 开局库集成

```toml
# ~/.config/chess-tui/openings.toml
[openings]
enabled = true
database = "~/.config/chess-tui/openings.db"
show_names = true
show_win_rates = true
```

---

## 自定义与主题

### 内置主题

```bash
# 列出所有主题
chess-tui --list-themes

# 可用主题:
# - default    经典黑白
# - blue       蓝色主题
# - green      绿色主题
# - gruvbox    Gruvbox 暗色
# - nord       Nord 主题
# - dracula    Dracula 主题
# - solarized  Solarized 主题
# - monokai    Monokai 主题
# - high-contrast  高对比度
```

### 应用主题

```bash
# 临时应用
chess-tui --theme gruvbox

# 永久设置
# ~/.config/chess-tui/config.toml
[display]
theme = "gruvbox"
```

### 自定义主题

```toml
# ~/.config/chess-tui/themes/my-theme.toml
name = "My Custom Theme"

[colors]
white_square = "#f0d9b5"
black_square = "#b58863"
white_piece = "#ffffff"
black_piece = "#000000"
selected = "#ff0000"
highlight = "#00ff00"
last_move = "#ffff00"
coordinates = "#808080"

[board]
show_coordinates = true
coordinate_color = "#808080"
show_highlights = true
show_legal_moves = true
```

### 棋盘样式

```toml
# ~/.config/chess-tui/config.toml
[board]
size = "medium"  # small, medium, large
unicode_pieces = true  # 使用 Unicode 棋子
ascii_fallback = false  # ASCII 回退
show_legal_hints = true  # 显示合法移动提示
```

### 键盘快捷键自定义

```toml
# ~/.config/chess-tui/config.toml
[keybindings]
# 导航
move_up = "k"
move_down = "j"
move_left = "h"
move_right = "l"

# 游戏操作
select = "Enter"
cancel = "Esc"
undo = "u"
resign = "Ctrl+R"
draw_offer = "Ctrl+D"
flip_board = "f"

# 界面
toggle_theme = "t"
show_help = "?"
toggle_coordinates = "c"
```

---

## 故障排除

### 常见问题

#### 1. "Command not found: chess-tui"

**原因**: 未安装或 PATH 未配置

**解决**:
```bash
# 验证安装
which chess-tui

# Cargo 安装后添加到 PATH
export PATH="$HOME/.cargo/bin:$PATH"

# 添加到 ~/.bashrc 或 ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### 2. "Failed to connect to Lichess"

**原因**: Token 无效或网络问题

**解决**:
```bash
# 验证 token
curl -H "Authorization: Bearer YOUR_TOKEN" https://lichess.org/api/account

# 检查网络
ping lichess.org

# 重新生成 token
# 访问 https://lichess.org/account/oauth/token
```

#### 3. "Engine not found"

**原因**: Stockfish 未安装或路径错误

**解决**:
```bash
# 安装 Stockfish
sudo apt install stockfish  # Linux
brew install stockfish      # macOS

# 验证路径
which stockfish

# 更新配置
# ~/.config/chess-tui/config.toml
[engine]
path = "/usr/bin/stockfish"  # 使用实际路径
```

#### 4. "Display issues" / 乱码

**原因**: 终端不支持 UTF-8 或 Unicode 棋子

**解决**:
```bash
# 设置 UTF-8
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# 使用替代终端
alacritty -e chess-tui
# 或
kitty chess-tui

# 禁用 Unicode 棋子
# ~/.config/chess-tui/config.toml
[display]
unicode_pieces = false
```

#### 5. 游戏崩溃/卡死

**原因**: 资源不足或 Bug

**解决**:
```bash
# 降低引擎深度
# ~/.config/chess-tui/config.toml
[engine]
depth = 10  # 降低到 10

# 禁用提前思考
ponder = false

# 报告 Bug
# https://github.com/thomas-mauran/chess-tui/issues
```

#### 6. PGN 无法加载

**原因**: PGN 格式错误或文件损坏

**解决**:
```bash
# 验证 PGN 格式
head -20 game.pgn

# 使用其他工具验证
pgnvalid game.pgn

# 转换格式
pgn-converter --input game.pgn --output fixed.pgn
```

#### 7. 输入延迟高

**原因**: 终端渲染慢或引擎占用

**解决**:
```bash
# 降低引擎深度
# ~/.config/chess-tui/config.toml
[engine]
depth = 10
threads = 1

# 禁用动画
[display]
animations = false

# 使用更快的终端
# alacritty, kitty, wezterm
```

#### 8. 颜色显示不正确

**原因**: 终端不支持真彩色

**解决**:
```bash
# 检查终端支持
echo $COLORTERM
# 应该是 "truecolor" 或 "24bit"

# 使用支持真彩色的终端
# alacritty, kitty, iTerm2, Windows Terminal

# 回退到 256 色主题
# ~/.config/chess-tui/config.toml
[display]
color_mode = "256"
```

### 性能优化

#### 降低资源使用

```toml
# ~/.config/chess-tui/config.toml
[engine]
depth = 12  # 降低深度
threads = 1  # 单线程
hash_size = 32  # 降低哈希表大小

[display]
animations = false  # 禁用动画
```

#### 提高响应速度

```toml
# 禁用实时评估
[analysis]
enabled = false

# 或仅在请求时评估
on_demand = true
```

---

## 完整示例

### 示例 1: 快速练习赛

```bash
# 启动并与中等难度 AI 对战
chess-tui

# 菜单选择:
# 1. vs Computer
# 2. 选择难度：Medium (5-6)
# 3. 选择颜色：White
# 4. 开始游戏
```

### 示例 2: 分析大师棋局

```bash
# 下载大师棋局
curl -o kasparov.pgn "https://example.com/kasparov-games.pgn"

# 加载并分析
chess-tui --load kasparov.pgn --analyze

# 逐回合查看最佳着法
# 使用引擎评估每个位置
```

### 示例 3: 在线 Blitz 对战

```bash
# 配置 Lichess token
# ~/.config/chess-tui/lichess.toml
[lichess]
token = "your_token"

# 启动并创建 Blitz 挑战
chess-tui
# 选择: vs Lichess → Create Challenge
# 设置：Blitz (3+2)
# 分享链接
```

### 示例 4: 自定义主题

```toml
# ~/.config/chess-tui/themes/custom.toml
name = "Midnight Chess"

[colors]
white_square = "#2c3e50"
black_square = "#34495e"
white_piece = "#ecf0f1"
black_piece = "#95a5a6"
selected = "#e74c3c"
highlight = "#2ecc71"
last_move = "#f39c12"

[board]
show_coordinates = true
coordinate_color = "#7f8c8d"
```

```bash
# 应用自定义主题
chess-tui --theme custom
```

### 示例 5: 批量分析比赛

```bash
#!/bin/bash
# tournament-analysis.sh

PGN_DIR="tournament_games"
OUTPUT_DIR="analysis_results"

mkdir -p "$OUTPUT_DIR"

for pgn in "$PGN_DIR"/*.pgn; do
    echo "Analyzing $(basename $pgn)..."
    chess-tui --load "$pgn" --analyze --depth 15 \
        --output "$OUTPUT_DIR/$(basename $pgn .pgn)_analysis.txt"
done

echo "Analysis complete! Results in $OUTPUT_DIR"
```

### 示例 6: 开局研究

```bash
#!/bin/bash
# opening-study.sh

# 创建开局 PGN 文件
cat > sicilian.pgn << 'EOF'
[Event "Study"]
[Opening "Sicilian Defense"]
1. e4 c5 2. Nf3 d6 3. d4 cxd4 4. Nxd4 Nf6 5. Nc3 a6
EOF

# 分析开局
chess-tui --load sicilian.pgn --analyze --depth 20

# 查看 Stockfish 评估
# 记录关键位置和最佳着法
```

---

## 性能基准

### 引擎性能测试

| 深度 | 每步时间 | Elo 近似 | 推荐用途 |
|------|---------|---------|---------|
| 5 | <0.1s | 800 | 快速练习 |
| 10 | 0.5s | 1400 | 休闲对战 |
| 15 | 2s | 1800 | 认真对弈 |
| 20 | 10s | 2200 | 深度分析 |
| 22 | 30s+ | 2400+ | 引擎对战 |

### 内存使用

| 配置 | 内存占用 |
|------|---------|
| 基础运行 | ~50MB |
| 引擎深度 10 | ~100MB |
| 引擎深度 20 | ~200MB |
| 多 PV 分析 | ~300MB |

### 启动时间

| 安装方式 | 启动时间 |
|---------|---------|
| 二进制 | <0.1s |
| Cargo | <0.1s |
| 源码编译 | <0.1s |

---

## 常见问题

### Q: chess-tui 与其他终端象棋游戏相比如何？

**A**: chess-tui 的优势:
- 现代 Rust 实现，内存安全
- 活跃的维护和更新
- Lichess 集成好
- 主题系统完善

替代方案:
- **ChessTui** (另一个项目)
- **GNU Chess** + 终端前端
- **Stockfish** + 自定义 UI

### Q: 如何备份我的棋局历史？

**A**:
```bash
# 棋局存储在
ls ~/.local/share/chess-tui/games/

# 备份
tar -czf chess-games-backup.tar.gz ~/.local/share/chess-tui/

# 恢复
tar -xzf chess-games-backup.tar.gz -C ~/
```

### Q: 可以录制游戏视频吗？

**A**:
- 使用终端录制工具 (asciinema)
- 或屏幕录制 (OBS)
- PGN 导出后可在其他平台分享

### Q: 支持变体象棋吗？

**A**:
- 目前仅支持标准国际象棋
- 变体 (如 Chess960) 计划中
- 关注 GitHub 仓库获取更新

### Q: 如何提交 Bug 或功能请求？

**A**:
1. 访问 https://github.com/thomas-mauran/chess-tui/issues
2. 搜索是否已有类似问题
3. 创建新 Issue，提供:
   - 系统信息
   - chess-tui 版本
   - 复现步骤
   - 预期行为 vs 实际行为

### Q: 可以贡献代码吗？

**A**:
欢迎贡献！
1. Fork 仓库
2. 创建功能分支
3. 实现功能/修复
4. 运行测试
5. 提交 PR

### Q: 支持蓝牙棋盘吗？

**A**:
- 目前不支持
- 可通过 DGT 电子棋盘间接支持
- 未来可能添加蓝牙支持

### Q: 如何在服务器上运行？

**A**:
```bash
# SSH 到服务器
ssh user@server

# 安装 chess-tui
cargo install chess-tui

# 运行 (需要支持 Unicode 的终端)
chess-tui
```

### Q: 支持残局数据库吗？

**A**:
- 目前不支持 Syzygy 等数据库
- 计划添加基础残局支持
- 欢迎贡献此功能

### Q: 可以自定义开局库吗？

**A**:
- 目前使用内置开局识别
- 自定义开局库计划中
- 可手动加载 PGN 学习

---

## 社区与资源

### 官方资源

- **GitHub**: https://github.com/thomas-mauran/chess-tui
- **Issues**: https://github.com/thomas-mauran/chess-tui/issues
- **Discussions**: https://github.com/thomas-mauran/chess-tui/discussions
- **Releases**: https://github.com/thomas-mauran/chess-tui/releases

### 相关项目

- **Stockfish**: https://stockfishchess.org/ - 最强开源国际象棋引擎
- **Lichess**: https://lichess.org/ - 免费国际象棋平台
- **UCI Protocol**: https://backscattering.de/chess/uci/ - 通用棋类接口协议

### 学习资源

- **Chess.com Lessons**: https://www.chess.com/lessons
- **Lichess Learning**: https://lichess.org/learn
- **Chess Tactics**: https://chesstempo.com/
- **Chess Programming Wiki**: https://www.chessprogramming.org/

### 社区

- **Reddit r/chess**: https://reddit.com/r/chess
- **Lichess Forum**: https://lichess.org/forum
- **Chess.com Forum**: https://chess.com/forum

---

*Community contribution - unofficial comprehensive guide*  
*Last updated: 2026-04-10*  
*Word count: ~16KB*
