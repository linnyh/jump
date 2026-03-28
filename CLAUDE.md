# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 快速命令

```bash
cargo build           # Debug 构建
cargo build --release # Release 构建
cargo test            # 运行所有测试
cargo test <name>     # 运行单个测试 (如 cargo test fuzzy_score)
cargo install --force # 安装/更新到 ~/.cargo/bin
```

## 架构

`j` 是一个目录跳转工具，二进制输出 `cd /path`，shell 插件负责实际执行。

### 核心模块

- `src/main.rs` — CLI 入口，使用 clap 解析参数
- `src/core/jumper.rs` — 生成 `cd /path` 输出
- `src/core/matcher.rs` — 模糊匹配（基于 `fuzzy-matcher` crate）
- `src/core/storage.rs` — 书签和历史的持久化（JSON）
- `src/commands/` — 各子命令实现（jump, add, rm, list, project 等）

### 关键设计

**Jump 匹配顺序**（`src/commands/jump.rs`）：
1. 本地目录匹配（当前目录下的子目录）
2. 书签匹配（名称 x2 权重，名称前缀 +500 bonus）
3. 会话历史匹配
4. 项目根目录匹配（向上查找 .git、Cargo.toml 等）

**项目根目录检测**（`src/commands/project.rs`）：
- 支持：.git, Cargo.toml, package.json, go.mod, pyproject.toml, pom.xml, build.gradle, CMakeLists.txt, Makefile, .svn, .hg
- `j root` 列出所有找到的项目根目录
- `j root <pattern>` 模糊匹配并跳转

**配置目录**（`src/config.rs`）：
- macOS: `~/Library/Application Support/jump/`
- Linux: `~/.config/jump/`
- 包含 `bookmarks.json` 和 `history.json`

**Shell 插件**（`shell/j.sh`）：
- source 到 shell 中拦截 `j` 命令
- 处理 `..`, `.`, `/path`, `-` 等 cd 风格路径
- `eval` 二进制输出的 `cd /path` 执行跳转

### 开发注意

- 书签名称前缀匹配优先于路径匹配（+500 bonus）
- `shell/j.sh` 需要同步到 `~/.cargo/bin/j.sh`
- `j -i` 优先使用 fzf，无 fzf 时自动降级为编号选择器
- Tab 补全读取 `bookmarks.json` 实时补全书签名称
