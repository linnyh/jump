# 🚀 j - 快速目录跳转工具

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="version">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="license">
  <img src="https://img.shields.io/badge/Rust-1.56+-orange.svg" alt="rust">
</p>

> ⚡ 让目录跳转像飞一样！j 是一个轻量级的命令行工具，结合书签管理和模糊匹配，让你的终端导航效率提升 10 倍。

## ✨ 特性

| 特性 | 说明 |
|------|------|
| 🎯 **CD 替代** | 完全替代 `cd`，支持 `..`、`/path`、`-`、`--back` 等 |
| 🔍 **模糊匹配** | FZF 风格的模糊搜索，输入即跳转 |
| 🔖 **书签管理** | 保存常用目录，支持分组分类 |
| 📜 **会话历史** | 自动记录会话访问过的目录 |
| 🗂️ **本地优先** | 优先匹配当前目录下的子目录 |
| 🌈 **跨平台** | 支持 macOS 和 Linux |

## ⚡ 快速开始

```bash
# 1. 安装
cargo install --path .

# 2. 配置 shell 插件（添加到 ~/.zshrc）
echo 'source /path/to/j/shell/j.sh' >> ~/.zshrc
source ~/.zshrc

# 3. 开始使用
cd ~/Projects/myapp
j add myapp          # 添加书签
j myapp              # 快速跳转回来
j ..                 # 跳转到父目录
j --back             # 返回
```

## 📦 安装

### 方式一：源码安装

```bash
cargo install --path .
```

### 方式二：Homebrew (待支持)

```bash
brew install j
```

### Shell 插件配置

> ⚠️ **重要**: cd 风格命令（`j ..`、`j --back` 等）需要加载 shell 插件才能工作。

在 `~/.zshrc` 或 `~/.bashrc` 中添加：

```bash
source /path/to/j/shell/j.sh
```

## 🎮 使用指南

### CD 风格命令（完全替代 cd）

| 命令 | 说明 |
|------|------|
| `j ..` | 跳转到父目录 |
| `j /path` | 跳转到绝对路径 |
| `j ../dir` | 跳转到相对路径 |
| `j -` | 返回上一个目录 |
| `j --back` / `j -b` | 返回上一次跳转的目录 |

### 🔖 书签管理

| 命令 | 说明 |
|------|------|
| `j add <name>` | 添加书签（当前目录） |
| `j add <name> --group <group>` | 添加书签到指定分组 |
| `j rm <name>` | 删除书签 |
| `j list` | 列出所有书签 |
| `j list --group <group>` | 列出指定分组的书签 |
| `j groups` | 列出所有分组 |

### 📋 其他命令

| 命令 | 说明 |
|------|------|
| `j <pattern>` | 模糊匹配并跳转 |
| `j hist` | 查看跳转历史 |
| `j recent` | 查看会话历史 |
| `j -i` | 交互式选择（需安装 fzf） |
| `j -e` | 编辑配置文件 |
| `j ~` | 跳转到主目录 |

## 🎯 匹配优先级

```
1️⃣  书签名称匹配（最高）
    ↓
2️⃣  书签路径匹配
    ↓
3️⃣  本地目录匹配
    ↓
4️⃣  会话历史匹配
```

## 💡 使用示例

```bash
# 📁 目录跳转
j ..                 # 父目录
j /Users             # 绝对路径
j ../var             # 相对路径
j -b                 # 返回上次跳转

# 🔖 书签管理
cd ~/Projects/work
j add project --group work      # 添加带分组书签

cd ~/Documents
j add notes --group personal   # 个人书签

# 📋 查看书签
j list                          # 列出所有书签
j list --group work             # 按分组查看
j groups                        # 查看所有分组

# 🚀 快速跳转
j proj                          # 模糊匹配书签
j doc                           # 本地优先
```

## ⚙️ 配置

**配置文件位置：**
- 🐧 Linux: `~/.config/ccd/`
- 🍎 macOS: `~/Library/Application Support/ccd/`

**文件说明：**
- `bookmarks.json` - 📚 书签数据
- `history.json` - 📝 跳转历史

可以直接编辑配置文件，或使用 `j -e` 打开编辑器修改。

## 🔧 依赖

| 依赖 | 说明 |
|------|------|
| [Rust](https://www.rust-lang.org/) 1.56+ | 编译环境 |
| [fzf](https://github.com/junegunn/fzf) | 交互式选择（可选） |

## 📄 License

MIT License - 随意使用 🚀

---

<div align="center">
  <p>Made with ❤️ by Rust</p>
</div>
