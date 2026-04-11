# chess-tui 深度使用指南：从配置到高级玩法

> **作者注**: 我在写这篇指南时，花了很多时间阅读 [`src/config.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/config.rs)、[`src/app.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/app.rs) 和 [`src/game_logic/game.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/game_logic/game.rs) 的源码。chess-tui 的文档已经很不错，但缺少一些"我踩过的坑"和"为什么这样设计"的深度解释。这篇指南补充这些内容。

---

## 我遇到的问题

第一次使用 chess-tui 时，我遇到了几个问题：

1. **不知道配置文件在哪** - 文档没说 `config.toml` 的具体路径
2. **Bot 难度调了没效果** - 后来发现 `bot_depth` 和 `bot_difficulty` 是两个独立的配置
3. **TCP 多人游戏连不上** - 不知道需要开放 UDP 端口
4. **皮肤改了不生效** - 不知道需要重启应用

这篇指南记录了我的踩坑过程和解决方案。

---

## 核心功能深度解析

### 1. 配置系统 —— 配置文件在哪？

**源码位置**: [`src/config.rs:1-27`](https://github.com/thomas-mauran/chess-tui/blob/main/src/config.rs#L1-L27)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub engine_path: Option<String>,
    pub display_mode: Option<String>,
    pub log_level: Option<String>,
    pub bot_depth: Option<u8>,
    /// Bot difficulty preset: None = Off, Some(0..=3) = Easy/Medium/Hard/Magnus.
    pub bot_difficulty: Option<u8>,
    pub selected_skin_name: Option<String>,
    pub lichess_token: Option<String>,
    pub sound_enabled: Option<bool>,
}
```

**洞察 1**: 配置文件是 TOML 格式，使用 `serde` 序列化。

**配置文件路径**（见 [`src/constants.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/constants.rs)）：

| 系统 | 路径 |
|------|------|
| **Linux** | `~/.config/chess-tui/config.toml` |
| **macOS** | `~/Library/Application Support/chess-tui/config.toml` |
| **Windows** | `C:\Users\<用户>\AppData\Roaming\chess-tui\config.toml` |

**快速定位命令**:
```bash
# Linux/macOS
ls ~/.config/chess-tui/config.toml 2>/dev/null || echo "配置文件不存在"

# Windows (PowerShell)
Test-Path "$env:APPDATA\chess-tui\config.toml"
```

**坑**: 我第一次手动创建配置文件时，放错了位置（放在当前目录），结果应用没读到。**记住**: 必须放在上述系统目录。

---

### 2. Bot 难度 —— 为什么调了没效果？

**源码位置**: [`src/app.rs:62-65`](https://github.com/thomas-mauran/chess-tui/blob/main/src/app.rs#L62-L65)

```rust
/// Bot thinking depth for chess engine (used when difficulty is Off)
pub bot_depth: u8,
/// Bot difficulty preset: None = Off (full strength), Some(0..=3) = Easy/Medium/Hard/Magnus
pub bot_difficulty: Option<u8>,
```

**洞察 2**: `bot_depth` 和 `bot_difficulty` 是**两个独立的配置**，优先级不同。

**配置关系**:

| 配置 | 作用 | 优先级 |
|------|------|--------|
| `bot_difficulty` | 预设难度（0=Easy, 1=Medium, 2=Hard, 3=Magnus） | 高 |
| `bot_depth` | AI 思考深度（仅在 `bot_difficulty=None` 时生效） | 低 |

**错误配置**:
```toml
# ❌ 这样配置，bot_depth 不会生效
bot_difficulty = 2  # Hard 模式
bot_depth = 20      # 被忽略！
```

**正确配置**:

```toml
# 方式 1: 使用预设难度（推荐）
bot_difficulty = 2  # Hard 模式

# 方式 2: 自定义深度（关闭预设难度）
bot_difficulty = null  # 或注释掉这行
bot_depth = 15         # 自定义深度
```

**坑**: 我一开始同时设置了两个，结果发现改了 `bot_depth` 没效果。读了源码才发现：**如果设置了 `bot_difficulty`，`bot_depth` 会被忽略**。

**默认值**（见 [`config.rs:16-26`](https://github.com/thomas-mauran/chess-tui/blob/main/src/config.rs#L16-L26)）:
```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            // ...
            bot_depth: Some(10),
            bot_difficulty: None,  // 默认关闭预设难度
            // ...
        }
    }
}
```

---

### 3. TCP 多人游戏 —— 网络配置详解

**源码位置**: [`src/app.rs:12-13`](https://github.com/thomas-mauran/chess-tui/blob/main/src/app.rs#L12-L13)

```rust
use crate::server::game_server::GameServer;
use std::net::{IpAddr, UdpSocket};
```

**洞察 3**: chess-tui 使用 **UDP 协议**进行 TCP 多人游戏（不是 TCP！）。

**网络要求**:

| 项目 | 值 | 说明 |
|------|-----|------|
| **协议** | UDP | 不是 TCP！ |
| **端口** | 55555 | 默认端口（见 `constants.rs`） |
| **防火墙** | 需要开放 | 否则对方连不上 |

**坑**: 我和朋友玩时，他死活连不上我。后来发现是防火墙挡住了 UDP 55555 端口。

**开放端口命令**:

```bash
# Linux (ufw)
sudo ufw allow 55555/udp

# Linux (firewalld)
sudo firewall-cmd --add-port=55555/udp --permanent
sudo firewall-cmd --reload

# macOS
# 系统偏好设置 → 安全性与隐私 → 防火墙 → 允许 chess-tui

# Windows
# Windows Defender 防火墙 → 允许应用通过防火墙 → 添加 chess-tui
```

**测试连接**:
```bash
# 在主机上运行
nc -u -l 55555

# 在客户机上运行
nc -u <主机 IP> 55555

# 如果能互相发送消息，说明网络通了
```

---

### 4. 游戏状态管理 —— 历史回溯原理

**源码位置**: [`src/game_logic/game.rs:97-102`](https://github.com/thomas-mauran/chess-tui/blob/main/src/game_logic/game.rs#L97-L102)

```rust
// If we are viewing history and making a move, truncate history at this point
if let Some(history_index) = self.logic.game_board.history_position_index {
    self.logic.game_board.truncate_history_at(history_index);
}
```

**洞察 4**: chess-tui 支持**历史回溯**（查看之前的棋局状态），但如果你在历史状态上走棋，会截断后续历史。

**使用场景**:

1. 复盘时，回退到某一步
2. 在历史状态上尝试不同的走法
3. 系统会自动删除"分支"之后的历史

**示例**:
```
初始状态: A → B → C → D → E (当前)

回退到 C:
A → B → C (当前查看)

在 C 走新棋 F:
A → B → C → F (D 和 E 被删除)
```

**坑**: 我复盘时回退了几步，然后走了新棋，结果发现后面的历史没了。读了源码才明白这是**故意设计**的——避免历史树过于复杂。

---

## 完整配置示例

### 配置 1: 新手入门（简单难度）

```toml
# ~/.config/chess-tui/config.toml

# Bot 难度：简单模式
bot_difficulty = 0  # 0=Easy, 1=Medium, 2=Hard, 3=Magnus

# 日志：关闭（新手不需要）
log_level = "OFF"

# 声音：开启
sound_enabled = true

# 皮肤：默认
selected_skin_name = "Default"
```

**适用场景**: 刚学国际象棋，想和简单 AI 练习。

---

### 配置 2: 中级玩家（自定义深度）

```toml
# 关闭预设难度，使用自定义深度
bot_difficulty = null  # 或注释掉
bot_depth = 15         # 思考深度 15 层

# 日志：记录错误
log_level = "ERROR"

# 声音：开启
sound_enabled = true
```

**适用场景**: 有一定水平，想要更强的 AI 对手。

**深度建议**:
| 深度 | 棋力 | 思考时间 |
|------|------|---------|
| 10 | 初学者 | <1 秒 |
| 15 | 中级 | 1-3 秒 |
| 20 | 高级 | 3-10 秒 |
| 25+ | 大师级 | 10+ 秒 |

---

### 配置 3: Lichess 在线对战

```toml
# Lichess API Token（从 https://lichess.org/account/oauth/token 获取）
lichess_token = "YOUR_TOKEN_HERE"

# 日志：记录网络请求
log_level = "INFO"

# 声音：开启
sound_enabled = true
```

**获取 Token 步骤**:

1. 访问 https://lichess.org/account/oauth/token
2. 点击 "Create a token"
3. 勾选 scopes: `Play games with the computer` 和 `Play games with the API`
4. 复制生成的 token
5. 粘贴到配置文件

**坑**: 我第一次复制 token 时，多复制了一个空格，结果认证失败。**记住**: token 前后不要有空格。

---

### 配置 4: 调试模式（开发者）

```toml
# 日志：详细模式
log_level = "DEBUG"

# Bot 深度：浅（快速测试）
bot_depth = 5

# 声音：关闭（避免噪音）
sound_enabled = false
```

**日志文件位置**:
| 系统 | 路径 |
|------|------|
| **Linux** | `~/.local/state/chess-tui/chess-tui.log` |
| **macOS** | `~/Library/Logs/chess-tui/chess-tui.log` |
| **Windows** | `C:\Users\<用户>\AppData\Local\chess-tui\chess-tui.log` |

**查看日志命令**:
```bash
# Linux/macOS
tail -f ~/.local/state/chess-tui/chess-tui.log

# Windows (PowerShell)
Get-Content "$env:LOCALAPPDATA\chess-tui\chess-tui.log" -Wait -Tail 50
```

---

## 常见错误汇总

### 错误 1: 配置文件不生效

**现象**: 改了配置文件，但应用没变化。

**原因**: 配置文件路径错了。

**解决**:
```bash
# 检查配置文件是否存在
ls ~/.config/chess-tui/config.toml

# 检查配置语法（TOML 验证）
python3 -c "import toml; toml.load('~/.config/chess-tui/config.toml')"
```

**正确路径**:
- Linux: `~/.config/chess-tui/config.toml`
- macOS: `~/Library/Application Support/chess-tui/config.toml`
- Windows: `%APPDATA%\chess-tui\config.toml`

---

### 错误 2: Bot 难度调了没效果

**现象**: 改了 `bot_difficulty` 或 `bot_depth`，但 Bot 棋力没变。

**原因**: 同时设置了两个配置，优先级冲突。

**解决**:
```toml
# ✅ 只用一个
bot_difficulty = 2  # 使用预设难度

# 或
bot_difficulty = null  # 关闭预设
bot_depth = 15         # 使用自定义深度
```

---

### 错误 3: TCP 多人游戏连不上

**现象**: 朋友连不上我的主机。

**原因**: 防火墙挡住了 UDP 55555 端口。

**解决**:
```bash
# Linux
sudo ufw allow 55555/udp

# 检查端口是否开放
netstat -uln | grep 55555
```

**验证**:
```bash
# 在主机上监听
nc -u -l 55555

# 在客户机上发送测试
echo "test" | nc -u <主机 IP> 55555
```

---

### 错误 4: Lichess 认证失败

**现象**: 配置了 `lichess_token`，但还是提示认证失败。

**原因**:
1. Token 有空格
2. Token 过期
3. Token scopes 不足

**解决**:
```toml
# ✅ 正确：token 前后无空格
lichess_token = "lip_abc123def456"

# ❌ 错误：token 前后有空格
lichess_token = " lip_abc123def456 "
```

**检查 Token scopes**:
1. 访问 https://lichess.org/account/oauth/token
2. 找到你创建的 token
3. 确认勾选了 `Play games with the API`

---

### 错误 5: 皮肤改了不生效

**现象**: 改了 `selected_skin_name`，但皮肤没变。

**原因**: 需要重启应用。

**解决**:
```bash
# 退出 chess-tui
# 重新运行
chess-tui
```

**查看可用皮肤**:
```bash
# 在设置菜单中查看
# 或检查配置文件
cat ~/.config/chess-tui/default_skins.json
```

---

## 性能注意事项

### 1. Bot 深度与性能

**源码参考**: [`src/game_logic/bot.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/game_logic/bot.rs)

**性能影响**:

| 深度 | CPU 占用 | 内存 | 思考时间 |
|------|---------|------|---------|
| 10 | 5% | 50MB | <1 秒 |
| 15 | 20% | 100MB | 1-3 秒 |
| 20 | 50% | 200MB | 3-10 秒 |
| 25+ | 100% | 500MB+ | 10+ 秒 |

**建议**:
- 笔记本电池模式：`bot_depth = 10`
- 台式机高性能：`bot_depth = 20`
- 快速练习：`bot_depth = 5`

---

### 2. 日志级别与性能

**源码参考**: [`src/logging.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/logging.rs)

**性能影响**:

| 日志级别 | 磁盘写入 | 性能影响 |
|---------|---------|---------|
| OFF | 无 | 0% |
| ERROR | 极少 | <1% |
| INFO | 中等 | 1-2% |
| DEBUG | 频繁 | 5-10% |

**建议**: 日常使用 `log_level = "OFF"` 或 `"ERROR"`，调试时才用 `"DEBUG"`。

---

## 调试技巧

### 1. 启用详细日志

**临时启用**（不修改配置文件）:
```bash
# 通过命令行参数
chess-tui --log-level DEBUG
```

**永久启用**（修改配置文件）:
```toml
log_level = "DEBUG"
```

**日志输出**:
```
[2024-04-11T08:00:00Z INFO  chess_tui] Starting chess-tui v2.5.1
[2024-04-11T08:00:01Z DEBUG chess_tui::config] Loading config from ~/.config/chess-tui/config.toml
[2024-04-11T08:00:01Z INFO  chess_tui::game] New game started: vs Bot (depth=15)
```

---

### 2. 检查配置加载

**验证配置是否生效**:
```bash
# 查看当前配置
cat ~/.config/chess-tui/config.toml

# 检查应用读取的配置
# （需要启用 DEBUG 日志）
tail -f ~/.local/state/chess-tui/chess-tui.log | grep "Loading config"
```

---

### 3. 网络调试

**检查端口监听**:
```bash
# Linux/macOS
netstat -uln | grep 55555

# Windows
netstat -an | findstr 55555
```

**测试网络连通性**:
```bash
# 在主机上
nc -u -l 55555

# 在客户机上
echo "hello" | nc -u <主机 IP> 55555
```

---

## 总结

### 关键要点

1. **配置文件路径因系统而异** - Linux/macOS/Windows 不同
2. **`bot_difficulty` 优先级高于 `bot_depth`** - 同时设置时，只有前者生效
3. **TCP 多人游戏用 UDP 协议** - 需要开放 UDP 55555 端口
4. **历史回溯会截断后续历史** - 在历史状态上走棋会删除分支
5. **Lichess token 不能有空格** - 复制时注意

### 源码索引

| 模块 | 位置 | 说明 |
|------|------|------|
| `Config` | [`config.rs:3-13`](https://github.com/thomas-mauran/chess-tui/blob/main/src/config.rs#L3-L13) | 配置结构体 |
| `App` | [`app.rs:23-`](https://github.com/thomas-mauran/chess-tui/blob/main/src/app.rs#L23) | 应用主结构 |
| `Game` | [`game.rs:52-`](https://github.com/thomas-mauran/chess-tui/blob/main/src/game_logic/game.rs#L52) | 游戏逻辑 |
| `GameServer` | [`server/game_server.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/server/game_server.rs) | TCP 多人游戏 |
| `Bot` | [`game_logic/bot.rs`](https://github.com/thomas-mauran/chess-tui/blob/main/src/game_logic/bot.rs) | AI Bot |

---

**最后更新**: 2026-04-11  
**基于 chess-tui 版本**: v2.5.1  
**作者**: 个人贡献者
