<div align="center">

# 🚀 Jump 快速目录跳转工具

<img src="img/icon.png" width="128" height="128" style="border-radius: 20%;">

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/linnyh/jump)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.56+-orange.svg)](https://www.rust-lang.org)

<p><b>让终端目录跳转像飞一样！</b></p>
<p>jump 是一个轻量级的命令行工具，结合智能书签管理和模糊匹配，让目录跳转效率提升 10 倍。</p>

</div>

---

## 为什么需要 jump ？

在日常开发中，我们经常需要：

- 在多个项目目录间频繁切换
- 记住常用目录的路径
- 快速跳转到深层嵌套的目录

**j** 可以帮你解决这些问题，只需记住几个简单的命令。

---

## 特性

| 特性 | 说明 |
|:---|:---|
| 🎯 **CD 替代** | 完全替代 `cd`，支持 `..`、`/path`、`-` 等所有常见语法 |
| 🔍 **模糊匹配** | FZF 风格的智能模糊搜索，输入即跳转 |
| 🔖 **书签系统** | 保存常用目录，支持分组管理 |
| 📜 **会话历史** | 自动记录会话访问过的目录 |
| ↩️ **返回功能** | `j --back` 一键返回上一次跳转的位置 |
| 🌈 **跨平台** | 支持 macOS 和 Linux |

---

## 快速开始

### 安装

```bash
# 方式一：下载预编译二进制（推荐，无需编译）
curl -L https://github.com/linnyh/jump/releases/download/v0.1.0/j -o /usr/local/bin/j
chmod +x /usr/local/bin/j

curl -L https://github.com/linnyh/jump/releases/download/v0.1.0/j.sh -o /usr/local/bin/j.sh
echo 'source /usr/local/bin/j.sh' >> ~/.zshrc
source ~/.zshrc

# 方式二：Homebrew
brew install linnyh/tap/jump
echo 'source $(brew --prefix)/opt/jump/share/jump/j.sh' >> ~/.zshrc
source ~/.zshrc

# 方式三：源码安装
git clone https://github.com/linnyh/jump.git
cd jump
cargo install --locked --path .
```

---

### 配置 Shell 插件

根据你的安装方式选择对应的配置：

```bash
# 方式一：预编译二进制安装
echo 'source /usr/local/bin/j.sh' >> ~/.zshrc

# 方式二：Homebrew 安装
echo 'source $(brew --prefix)/opt/jump/share/jump/j.sh' >> ~/.zshrc

# 方式三：源码安装
echo 'source /path/to/jump/shell/j.sh' >> ~/.zshrc

# 最后加载配置
source ~/.zshrc
```

---

## 使用指南

### 📁 目录跳转（替代 cd）

```bash
j ..                 # 跳转到父目录
j ../config          # 跳转到相对路径
j /Users/admin       # 跳转到绝对路径
j -                  # 返回上一个目录（等同于 cd -）
j --back             # 返回上一次跳转的位置（j 特有的功能）
j -b                 # --back 的简写
```

### 🔖 书签管理

```bash
# 添加书签
cd ~/Projects/myapp
j -a myapp                    # 添加书签，名称为 myapp
j -a work --group personal    # 添加到 personal 分组

# 查看书签
j -l                         # 列出所有书签
j list --group personal        # 只查看 personal 分组的书签
j -g                          # 列出所有分组

# 删除书签
j -d myapp                     # 删除名为 myapp 的书签
```

### 🔍 模糊跳转

```bash
j myapp              # 模糊匹配书签名称
j proj               # 自动匹配最相关的目录
j doc                # 匹配当前目录下的 docs 子目录
j                    # 不带参数：显示会话历史
```

### 📋 其他命令

```bash
j ~                  # 跳转到主目录
j -H                 # 查看跳转历史
j -r                 # 查看会话历史
j -i                 # 交互式选择（有 fzf 用 fzf，否则用编号选择）
j -e                 # 用编辑器打开配置文件
j -R                 # 列出所有项目根目录（.git、Cargo.toml 等）
j -R myapp           # 跳转到指定项目根目录
j --help             # 显示帮助信息
```

---

## 工作原理

### 匹配优先级

当输入 `j proj` 时，j 按以下顺序匹配：

```
1️⃣ 书签名称匹配（权重 x2 + 名称前缀 bonus）
   ↓ 如果匹配到 "proj" 或 "project" 书签
2️⃣ 书签路径匹配
   ↓ 在书签路径中查找 "proj"
3️⃣ 本地目录匹配
   ↓ 在当前目录下查找名为 "proj" 的子目录
4️⃣ 会话历史匹配
   ↓ 在会话历史中查找 "proj"
5️⃣ 项目根目录匹配
   ↓ 向上查找 .git、Cargo.toml 等项目标记
```

### 会话历史 vs 书签

| 类型 | 说明 | 持久化 |
|:---|:---|:---|
| **书签** | 手动添加的固定目录 | 持久保存 |
| **会话历史** | 自动记录的访问记录 | 仅当前会话有效 |

---

## 配置

### 配置文件位置

| 系统 | 路径 |
|:---|:---|
| macOS | `~/Library/Application Support/jump/` |
| Linux | `~/.config/jump/` |

### 配置文件说明

```bash
jump/
├── bookmarks.json    # 书签数据
└── history.json      # 跳转历史
```

### 直接编辑配置

```bash
# 方式一：用编辑器打开
j -e

# 方式二：直接编辑
vim ~/Library/Application\ Support/jump/bookmarks.json
```

### bookmarks.json 示例

```json
{
  "groups": ["work", "personal"],
  "work-project": {
    "path": "/Users/admin/Projects/work",
    "group": "work"
  },
  "docs": {
    "path": "/Users/admin/Documents",
    "group": "personal"
  },
  "home": {
    "path": "/Users/admin"
  }
}
```

---

## 常见问题

### Q: `j --back` 和 `cd -` 有什么区别？

**`cd -`** 返回上一个目录（基于 shell 的目录栈）。

**`j --back`** 返回上一次使用 j 跳转的位置。

```
当前: /home/user/project
执行: j work        → 跳转到 /home/user/work
执行: j --back       → 返回 /home/user/project (j 记录的上一次位置)
执行: cd ..          → 跳转到 /home/user
执行: cd -           → 返回 /home/user/project (cd 的目录栈)
执行: j --back       → 仍返回 /home/user/work (j 的跳转记录)
```

### Q: 如何在书签中使用分组？

分组可以帮你组织书签：

```bash
# 添加带分组的书签
cd ~/Projects/client-a
j add client-a --group work

cd ~/Documents/notes
j add notes --group personal

# 按分组查看
j list --group work        # 只看 work 分组

# 查看所有分组
j groups
```

### Q: 模糊匹配是如何工作的？

j 使用类似 fzf 的模糊匹配算法：

```
输入 "myapp" 可以匹配：
- myapp          ✓ 精确匹配
- my-app         ✓ 模糊匹配
- m_y_a_p_p      ✓ 首字母匹配
- mp             ✓ 子序列匹配
```

---

## 依赖

| 依赖 | 版本 | 说明 |
|:---|:---|:---|
| Rust | 1.56+ | 编译环境 |
| fzf | 最新 | 交互式选择（可选）|

---

## License

MIT License - 随意使用 🚀

---

<div align="center">

**Made with ❤️ using Rust**

</div>
