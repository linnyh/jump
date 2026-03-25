<div align="center">

# 🚀 j - 快速目录跳转工具

<p><img src="img/icon.png" width="128" height="128" style="border-radius: 20%;"></p>

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.56+-orange.svg)](https://www.rust-lang.org)

<p><i>⚡ 让目录跳转像飞一样！j 是一个轻量级的命令行工具，结合书签管理和模糊匹配，让你的终端导航效率提升 10 倍。</i></p>

</div>

---

## ✨ 特性

| | |
|:---|:---|
| 🎯 **CD 替代** | 完全替代 `cd`，支持 `..`、`/path`、`-`、`--back` 等 |
| 🔍 **模糊匹配** | FZF 风格的模糊搜索，输入即跳转 |
| 🔖 **书签管理** | 保存常用目录，支持分组分类 |
| 📜 **会话历史** | 自动记录会话访问过的目录 |
| 🗂️ **本地优先** | 优先匹配当前目录下的子目录 |
| 🌈 **跨平台** | 支持 macOS 和 Linux |

---

## ⚡ 快速开始

```bash
# 1. 安装
cargo install --path .

# 2. 配置 shell 插件
echo 'source /path/to/jump/shell/j.sh' >> ~/.zshrc
source ~/.zshrc

# 3. 开始使用
cd ~/Projects/myapp
j add myapp          # 添加书签
j myapp              # 快速跳转回来
j ..                 # 跳转到父目录
j --back             # 返回
```

---

## 📦 安装

### 源码安装

```bash
cargo install --path .
```

### Shell 插件配置

> ⚠️ **重要**: cd 风格命令（`j ..`、`j --back` 等）需要加载 shell 插件才能工作。

```bash
# 添加到 ~/.zshrc
source /path/to/jump/shell/j.sh
```

---

## 🎮 使用指南

### CD 风格命令

| 命令 | 说明 |
|:---|:---|
| `j ..` | 跳转到父目录 |
| `j /path` | 跳转到绝对路径 |
| `j ../dir` | 跳转到相对路径 |
| `j -` | 返回上一个目录 |
| `j --back` / `j -b` | 返回上一次跳转的目录 |

### 🔖 书签管理

| 命令 | 说明 |
|:---|:---|
| `j add <name>` | 添加书签 |
| `j add <name> --group <group>` | 添加到分组 |
| `j rm <name>` | 删除书签 |
| `j list` | 列出所有书签 |
| `j list --group <group>` | 按分组查看 |
| `j groups` | 列出所有分组 |

### 📋 其他命令

| 命令 | 说明 |
|:---|:---|
| `j <pattern>` | 模糊匹配跳转 |
| `j hist` | 查看跳转历史 |
| `j recent` | 查看会话历史 |
| `j -i` | 交互式选择 |
| `j -e` | 编辑配置文件 |

---

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

---

## 💡 使用示例

```bash
# 📁 目录跳转
j ..                 # 父目录
j /Users             # 绝对路径
j ../var             # 相对路径
j -b                 # 返回上次跳转

# 🔖 书签管理
cd ~/Projects/work
j add project --group work

cd ~/Documents
j add notes --group personal

# 📋 查看书签
j list                          # 列出所有
j list --group work            # 按分组
j groups                       # 所有分组

# 🚀 快速跳转
j proj                          # 模糊匹配
```

---

## ⚙️ 配置

**配置文件位置：**
- 🐧 Linux: `~/.config/jump/`
- 🍎 macOS: `~/Library/Application Support/jump/`

**文件说明：**
- `bookmarks.json` - 📚 书签数据
- `history.json` - 📝 跳转历史

---

## 🔧 依赖

| 依赖 | 说明 |
|:---|:---|
| [Rust](https://www.rust-lang.org/) 1.56+ | 编译环境 |
| [fzf](https://github.com/junegunn/fzf) | 交互式选择（可选）|

---

<div align="center">

**MIT License** · Made with ❤️ by Rust

</div>
