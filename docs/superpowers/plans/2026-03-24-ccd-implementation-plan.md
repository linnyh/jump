# ccd 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**目标：** 构建一个 Rust 实现的 CLI 工具，通过模糊匹配、书签管理和历史记录实现快速目录跳转。

**架构概述：** 采用单仓库 + 子命令模式，使用 Clap 进行 CLI 参数解析，serde_json 处理数据持久化，自实现 FZF 风格的模糊匹配算法。数据存储在 `~/.config/ccd/` 目录下。

**技术栈：** Rust, Clap, serde_json

---

## 项目文件结构

```
ccd/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI 入口，命令路由
│   ├── config.rs            # 配置路径管理
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── jump.rs           # 跳转逻辑
│   │   ├── add.rs            # 添加书签
│   │   ├── rm.rs             # 删除书签
│   │   ├── list.rs           # 列出书签
│   │   └── hist.rs           # 历史记录
│   └── core/
│       ├── mod.rs
│       ├── storage.rs        # JSON 读写
│       ├── matcher.rs        # 模糊匹配算法
│       └── jumper.rs         # 跳转执行
└── shell/
    └── ccd.sh                # 可选 shell 插件
```

---

## 任务分解

### Task 1: 项目初始化

**Files:**
- Create: `ccd/Cargo.toml`
- Create: `ccd/src/main.rs`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "ccd"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"
```

- [ ] **Step 2: 创建 main.rs 骨架**

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ccd")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Parser, Debug)]
enum Command {
    Add { name: String },
    Rm { name: String },
    List,
    Hist,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
```

- [ ] **Step 3: 验证编译**

Run: `cd ccd && cargo build`
Expected: 编译成功，无错误

- [ ] **Step 4: 提交**

```bash
git add ccd/
git commit -m "feat: initialize ccd project with Cargo.toml and main.rs skeleton"
```

---

### Task 2: 配置模块

**Files:**
- Create: `ccd/src/config.rs`

- [ ] **Step 1: 编写测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir() {
        let config = Config::new();
        assert!(config.config_dir().ends_with(".config/ccd"));
    }

    #[test]
    fn test_bookmarks_path() {
        let config = Config::new();
        assert!(config.bookmarks_path().ends_with("bookmarks.json"));
    }

    #[test]
    fn test_history_path() {
        let config = Config::new();
        assert!(config.history_path().ends_with("history.json"));
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cd ccd && cargo test test_config_dir`
Expected: FAIL - Config 未定义

- [ ] **Step 3: 实现 Config 模块**

```rust
use std::path::PathBuf;

pub struct Config {
    config_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ccd");
        Self { config_dir }
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn bookmarks_path(&self) -> PathBuf {
        self.config_dir.join("bookmarks.json")
    }

    pub fn history_path(&self) -> PathBuf {
        self.config_dir.join("history.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: 运行测试验证通过**

Run: `cd ccd && cargo test`
Expected: PASS

- [ ] **Step 5: 提交**

```bash
git add ccd/src/config.rs ccd/src/main.rs
git commit -m "feat: add Config module for path management"
```

---

### Task 3: 存储模块 (JSON 读写)

**Files:**
- Create: `ccd/src/core/mod.rs`
- Create: `ccd/src/core/storage.rs`

- [ ] **Step 1: 编写测试**

```rust
// 在 storage.rs 中
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmarks_empty() {
        let bookmarks = Bookmarks { bookmarks: std::collections::HashMap::new() };
        assert!(bookmarks.get("test").is_none());
    }

    #[test]
    fn test_bookmarks_insert() {
        let mut bookmarks = Bookmarks { bookmarks: std::collections::HashMap::new() };
        bookmarks.bookmarks.insert("proj".to_string(), "/path/to/proj".to_string());
        assert_eq!(bookmarks.get("proj"), Some(&"/path/to/proj".to_string()));
    }

    #[test]
    fn test_history_entry_default() {
        let entry = HistoryEntry::new("/test/path");
        assert_eq!(entry.access_count, 0);
        assert!(entry.path.ends_with("/test/path"));
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cd ccd && cargo test storage`
Expected: FAIL - 模块不存在

- [ ] **Step 3: 创建 core/mod.rs**

```rust
pub mod storage;
pub mod matcher;
pub mod jumper;
```

- [ ] **Step 4: 实现 storage.rs**

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmarks {
    #[serde(flatten)]
    pub bookmarks: HashMap<String, String>,
}

impl Bookmarks {
    pub fn new() -> Self {
        Self { bookmarks: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.bookmarks.get(name)
    }

    pub fn insert(&mut self, name: String, path: String) {
        self.bookmarks.insert(name, path);
    }

    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.bookmarks.remove(name)
    }

    pub fn values(&self) -> Vec<&String> {
        self.bookmarks.values().collect()
    }

    pub fn len(&self) -> usize {
        self.bookmarks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bookmarks.is_empty()
    }
}

impl Default for Bookmarks {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub path: String,
    pub access_count: u32,
    #[serde(rename = "last_access")]
    pub last_access: String,
}

impl HistoryEntry {
    pub fn new(path: &str) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            path: path.to_string(),
            access_count: 0,
            last_access: chrono_timestamp(now),
        }
    }

    pub fn increment_access(&mut self) {
        self.access_count += 1;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.last_access = chrono_timestamp(now);
    }
}

fn chrono_timestamp(secs: u64) -> String {
    use std::time::UNIX_EPOCH;
    let dt = UNIX_EPOCH + std::time::Duration::from_secs(secs);
    let datetime = chrono::DateTime::<chrono::Utc>::from(dt);
    datetime.to_rfc3339()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_or_update(&mut self, path: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.path == path) {
            entry.increment_access();
        } else {
            self.entries.push(HistoryEntry::new(path));
        }
        // 按访问次数降序排序
        self.entries.sort_by(|a, b| b.access_count.cmp(&a.access_count));
    }

    pub fn recent(&self, n: usize) -> Vec<&HistoryEntry> {
        self.entries.iter().take(n).collect()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 5: 添加依赖到 Cargo.toml**

```toml
[dependencies]
chrono = "0.4"
```

- [ ] **Step 6: 运行测试验证通过**

Run: `cd ccd && cargo test`
Expected: PASS

- [ ] **Step 7: 提交**

```bash
git add ccd/src/core/ ccd/Cargo.toml
git commit -m "feat: add storage module with Bookmarks and History structs"
```

---

### Task 4: 模糊匹配算法

**Files:**
- Create: `ccd/src/core/matcher.rs`

- [ ] **Step 1: 编写测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match_high_score() {
        let score = fuzzy_score("project", "project");
        let partial = fuzzy_score("proj", "project");
        assert!(score > partial);
    }

    #[test]
    fn test_consecutive_match_bonus() {
        // "pro" 在 "project" 中连续匹配
        let consecutive = fuzzy_score("pro", "project");
        // "prj" 在 "project" 中分散匹配
        let scattered = fuzzy_score("prj", "project");
        assert!(consecutive > scattered);
    }

    #[test]
    fn test_no_match() {
        let score = fuzzy_score("xyz", "project");
        assert_eq!(score, 0);
    }

    #[test]
    fn test_camel_case_match() {
        let score = fuzzy_score("MP", "MyProject");
        assert!(score > 0);
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cd ccd && cargo test matcher`
Expected: FAIL - matcher 模块为空

- [ ] **Step 3: 实现 matcher.rs (简化版 FZF 算法)**

```rust
/// FZF 风格的模糊匹配评分
/// 返回得分：0 表示不匹配，正数表示匹配程度
pub fn fuzzy_score(input: &str, target: &str) -> u32 {
    let input_chars: Vec<char> = input.to_lowercase().chars().collect();
    let target_chars: Vec<char> = target.to_lowercase().chars().collect();

    if input_chars.is_empty() {
        return 0;
    }

    let mut score: u32 = 0;
    let mut input_idx = 0;
    let mut prev_match_idx: Option<usize> = None;
    let mut consecutive_bonus = 0;

    for (target_idx, tc) in target_chars.iter().enumerate() {
        if input_idx >= input_chars.len() {
            break;
        }

        if *tc == input_chars[input_idx] {
            // 基础得分
            score += 10;
            input_idx += 1;

            // 连续匹配加成
            if let Some(prev) = prev_match_idx {
                if target_idx == prev + 1 {
                    consecutive_bonus += 5;
                    score += consecutive_bonus;
                } else {
                    consecutive_bonus = 0;
                }
            }

            // 首字母加成
            if target_idx == 0 {
                score += 15;
            }

            // 单词边界加成
            if target_idx > 0 && target_chars[target_idx - 1] == '/' {
                score += 8;
            }

            prev_match_idx = Some(target_idx);
        }
    }

    // 所有输入字符都必须匹配
    if input_idx < input_chars.len() {
        return 0;
    }

    // 长度惩罚：短输入匹配长目标应该扣分
    if target.len() > input.len() * 3 {
        score = score.saturating_sub(5);
    }

    score
}

/// 匹配结果
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub path: String,
    pub score: u32,
}

/// 从列表中模糊匹配并排序
pub fn fuzzy_match<'a>(input: &str, candidates: &'a [&str]) -> Vec<MatchResult> {
    let mut results: Vec<MatchResult> = candidates
        .iter()
        .filter_map(|&path| {
            let score = fuzzy_score(input, path);
            if score > 0 {
                Some(MatchResult { path: path.to_string(), score })
            } else {
                None
            }
        })
        .collect();

    // 按得分降序排序
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}
```

- [ ] **Step 4: 运行测试验证通过**

Run: `cd ccd && cargo test matcher`
Expected: PASS

- [ ] **Step 5: 提交**

```bash
git add ccd/src/core/matcher.rs
git commit -m "feat: implement fuzzy matching algorithm (FZF style)"
```

---

### Task 5: 跳转执行模块

**Files:**
- Create: `ccd/src/core/jumper.rs`

- [ ] **Step 1: 编写测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_cd_command() {
        let cmd = build_cd_command("/home/user");
        assert!(cmd.ends_with("cd /home/user"));
    }

    #[test]
    fn test_print_cd_script() {
        // 测试输出格式
        let output = generate_cd_script("/test/path");
        assert!(output.contains("cd /test/path"));
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cd ccd && cargo test jumper`
Expected: FAIL

- [ ] **Step 3: 实现 jumper.rs**

```rust
use std::path::Path;

/// 生成 cd 命令脚本（用于 shell eval）
pub fn generate_cd_script(path: &str) -> String {
    // 需要处理路径中的特殊字符
    format!("cd {}", shell_escape(path))
}

/// 简单的 shell 转义
fn shell_escape(path: &str) -> String {
    // 如果路径不包含单引号和空格，直接返回
    if !path.contains(' ') && !path.contains('\'') && !path.contains('\\') {
        return path.to_string();
    }

    // 使用单引号转义，将单引号替换为 '\'' (结束单引号，添加转义单引号，重新开始单引号)
    let escaped = path.replace('\'', "'\\''");
    format!("'{}'", escaped)
}

/// 验证目录是否存在
pub fn validate_path(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// 获取最佳匹配路径
pub fn select_best_match<'a>(matches: &[super::matcher::MatchResult]) -> Option<&'a str> {
    if matches.is_empty() {
        return None;
    }
    // 返回得分最高且目录存在的
    for m in matches {
        if validate_path(&m.path) {
            return Some(&m.path);
        }
    }
    // 如果都没有目录存在，返回第一个
    Some(&matches[0].path)
}
```

- [ ] **Step 4: 运行测试验证通过**

Run: `cd ccd && cargo test jumper`
Expected: PASS

- [ ] **Step 5: 提交**

```bash
git add ccd/src/core/jumper.rs
git commit -m "feat: add jumper module for cd command generation"
```

---

### Task 6: 主命令实现 - Jump (跳转)

**Files:**
- Create: `ccd/src/commands/mod.rs`
- Create: `ccd/src/commands/jump.rs`
- Modify: `ccd/src/main.rs`

- [ ] **Step 1: 编写测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_command_execute_no_args() {
        let cmd = JumpCommand { pattern: None };
        // 无参数时应该返回帮助或错误
        assert!(cmd.pattern.is_none());
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cd ccd && cargo test jump`
Expected: FAIL

- [ ] **Step 3: 创建 commands/mod.rs**

```rust
pub mod add;
pub mod list;
pub mod hist;
pub mod rm;
pub mod jump;

pub use add::AddCommand;
pub use list::ListCommand;
pub use hist::HistCommand;
pub use rm::RmCommand;
pub use jump::JumpCommand;
```

- [ ] **Step 4: 实现 commands/jump.rs**

```rust
use crate::core::{matcher, storage, Config};

pub struct JumpCommand {
    pub pattern: Option<String>,
}

impl JumpCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        // 无参数时显示帮助
        let Some(pattern) = &self.pattern else {
            return Err("Usage: ccd <pattern> - jump to a directory".to_string());
        };

        // 加载书签
        let bookmarks = storage::load_bookmarks(config)?;

        // 收集所有目录候选
        let candidates: Vec<&str> = bookmarks.values().map(|s| s.as_str()).collect();

        // 模糊匹配
        let matches = matcher::fuzzy_match(pattern, &candidates);

        if matches.is_empty() {
            return Err("No matching directory found".to_string());
        }

        // 选择最佳匹配
        let best = matches.first().ok_or("No match")?;

        // 验证目录存在
        if !std::path::Path::new(&best.path).is_dir() {
            return Err(format!("Directory does not exist: {}", best.path));
        }

        // 输出 cd 命令（用户需要 eval）
        println!("{}", crate::core::jumper::generate_cd_script(&best.path));
        Ok(())
    }
}
```

- [ ] **Step 5: 更新 main.rs 添加 Jump 子命令**

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ccd")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    /// Jump to directory matching pattern
    pattern: Option<String>,
}

#[derive(Parser, Debug)]
enum Command {
    /// Add a bookmark
    Add { name: String },
    /// Remove a bookmark
    Rm { name: String },
    /// List all bookmarks
    List,
    /// Show history
    Hist,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();

    match cli.command {
        Some(Command::Add { name }) => {
            AddCommand { name }.execute(&config).unwrap();
        }
        Some(Command::Rm { name }) => {
            RmCommand { name }.execute(&config).unwrap();
        }
        Some(Command::List) => {
            ListCommand.execute(&config).unwrap();
        }
        Some(Command::Hist) => {
            HistCommand.execute(&config).unwrap();
        }
        None => {
            JumpCommand { pattern: cli.pattern }.execute(&config).unwrap();
        }
    }
}
```

- [ ] **Step 6: 添加缺失的 use 语句到 main.rs**

```rust
use crate::{AddCommand, Config, HistCommand, JumpCommand, ListCommand, RmCommand};
```

- [ ] **Step 7: 运行测试验证编译**

Run: `cd ccd && cargo build`
Expected: 编译成功

- [ ] **Step 8: 提交**

```bash
git add ccd/src/commands/ ccd/src/main.rs
git commit -m "feat: implement jump command with fuzzy matching"
```

---

### Task 7: Add 命令实现

**Files:**
- Create: `ccd/src/commands/add.rs`

- [ ] **Step 1: 编写测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_command_has_name() {
        let cmd = AddCommand { name: "myproj".to_string() };
        assert_eq!(cmd.name, "myproj");
    }
}
```

- [ ] **Step 2: 实现 add.rs**

```rust
use crate::core::storage;
use crate::Config;

pub struct AddCommand {
    pub name: String,
}

impl AddCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;

        let current_path = current_dir.to_string_lossy().to_string();

        // 加载现有书签
        let mut bookmarks = storage::load_bookmarks(config)?;

        // 添加新书签
        bookmarks.insert(self.name.clone(), current_path.clone());

        // 保存
        storage::save_bookmarks(config, &bookmarks)?;

        println!("Added bookmark '{}' -> {}", self.name, current_path);
        Ok(())
    }
}
```

- [ ] **Step 3: 提交**

```bash
git add ccd/src/commands/add.rs
git commit -m "feat: implement add command for bookmarks"
```

---

### Task 8: List 命令实现

**Files:**
- Create: `ccd/src/commands/list.rs`

- [ ] **Step 1: 实现 list.rs**

```rust
use crate::core::storage;
use crate::Config;

pub struct ListCommand;

impl ListCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let bookmarks = storage::load_bookmarks(config)?;

        if bookmarks.is_empty() {
            println!("No bookmarks yet. Use 'ccd add <name>' to add one.");
            return Ok(());
        }

        for (name, path) in &bookmarks.bookmarks {
            println!("{} -> {}", name, path);
        }

        Ok(())
    }
}
```

- [ ] **Step 2: 提交**

```bash
git add ccd/src/commands/list.rs
git commit -m "feat: implement list command for bookmarks"
```

---

### Task 9: Rm 命令实现

**Files:**
- Create: `ccd/src/commands/rm.rs`

- [ ] **Step 1: 实现 rm.rs**

```rust
use crate::core::storage;
use crate::Config;

pub struct RmCommand {
    pub name: String,
}

impl RmCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let mut bookmarks = storage::load_bookmarks(config)?;

        if let Some(removed) = bookmarks.remove(&self.name) {
            storage::save_bookmarks(config, &bookmarks)?;
            println!("Removed bookmark '{}' -> {}", self.name, removed);
            Ok(())
        } else {
            Err(format!("Bookmark '{}' not found", self.name))
        }
    }
}
```

- [ ] **Step 2: 提交**

```bash
git add ccd/src/commands/rm.rs
git commit -m "feat: implement rm command for bookmarks"
```

---

### Task 10: Hist 命令实现

**Files:**
- Create: `ccd/src/commands/hist.rs`

- [ ] **Step 1: 实现 hist.rs**

```rust
use crate::core::storage;
use crate::Config;

pub struct HistCommand;

impl HistCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let history = storage::load_history(config)?;

        if history.entries.is_empty() {
            println!("No history yet. Use 'ccd <pattern>' to jump and build history.");
            return Ok(());
        }

        for entry in history.entries.iter().take(20) {
            println!("{} [{}]: {}", entry.access_count, entry.last_access, entry.path);
        }

        Ok(())
    }
}
```

- [ ] **Step 2: 提交**

```bash
git add ccd/src/commands/hist.rs
git commit -m "feat: implement hist command for history"
```

---

### Task 11: Storage 完善 - 加载/保存功能

**Files:**
- Modify: `ccd/src/core/storage.rs`

- [ ] **Step 1: 编写测试**

```rust
#[test]
fn test_load_nonexistent_file() {
    let config = Config::new();
    // 应该返回空 Bookmarks 而不报错
    let result = load_bookmarks(&config);
    assert!(result.is_ok());
}
```

- [ ] **Step 2: 实现完整的 storage.rs 加载/保存逻辑**

```rust
use std::fs;
use std::path::Path;

// ... 现有结构 ...

pub fn load_bookmarks(config: &Config) -> Result<Bookmarks, String> {
    let path = config.bookmarks_path();

    if !path.exists() {
        return Ok(Bookmarks::new());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read bookmarks: {}", e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse bookmarks: {}", e))
}

pub fn save_bookmarks(config: &Config, bookmarks: &Bookmarks) -> Result<(), String> {
    let path = config.bookmarks_path();

    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 备份旧文件
    if path.exists() {
        let backup_path = format!("{}.bak", path.display());
        let _ = fs::copy(&path, &backup_path);
    }

    let content = serde_json::to_string_pretty(bookmarks)
        .map_err(|e| format!("Failed to serialize bookmarks: {}", e))?;

    fs::write(&path, content)
        .map_err(|e| format!("Failed to write bookmarks: {}", e))
}

pub fn load_history(config: &Config) -> Result<History, String> {
    let path = config.history_path();

    if !path.exists() {
        return Ok(History::new());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read history: {}", e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse history: {}", e))
}

pub fn save_history(config: &Config, history: &History) -> Result<(), String> {
    let path = config.history_path();

    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 备份旧文件
    if path.exists() {
        let backup_path = format!("{}.bak", path.display());
        let _ = fs::copy(&path, &backup_path);
    }

    let content = serde_json::to_string_pretty(history)
        .map_err(|e| format!("Failed to serialize history: {}", e))?;

    fs::write(&path, content)
        .map_err(|e| format!("Failed to write history: {}", e))
}
```

- [ ] **Step 3: 运行测试**

Run: `cd ccd && cargo test`
Expected: PASS

- [ ] **Step 4: 提交**

```bash
git add ccd/src/core/storage.rs
git commit -m "feat: implement storage load/save functions with backup"
```

---

### Task 12: Shell 插件

**Files:**
- Create: `ccd/shell/ccd.sh`

- [ ] **Step 1: 创建 shell/ccd.sh**

```bash
#!/bin/bash
# ccd shell plugin
# 在 .zshrc 或 .bashrc 中 source 此文件

ccd() {
    local result
    result=$(command ccd "$@")
    if [[ -n "$result" ]]; then
        eval "$result"
    fi
}

# Tab 补全 (仅 zsh)
if [[ -n "$ZSH_VERSION" ]]; then
    _ccd() {
        _arguments '1: :->commands' '*: :->args'
        case $state in
            commands)
                _describe 'command' '(
                    add:"Add bookmark for current directory"
                    rm:"Remove a bookmark"
                    list:"List all bookmarks"
                    hist:"Show jump history"
                )'
                ;;
        esac
    }
    compdef _ccd ccd
fi
```

- [ ] **Step 2: 设置执行权限**

Run: `chmod +x ccd/shell/ccd.sh`

- [ ] **Step 3: 提交**

```bash
git add ccd/shell/ccd.sh
git commit -m "feat: add shell plugin for zsh/bash integration"
```

---

### Task 13: 最终测试与清理

**Files:**
- Verify: 所有源文件

- [ ] **Step 1: 运行完整测试套件**

Run: `cd ccd && cargo test`
Expected: 全部 PASS

- [ ] **Step 2: 构建 release 版本**

Run: `cd ccd && cargo build --release`
Expected: 编译成功

- [ ] **Step 3: 测试 CLI 帮助**

Run: `cd ccd && cargo run -- --help`
Expected: 显示帮助信息

- [ ] **Step 4: 提交**

```bash
git add -A
git commit -m "feat: complete ccd CLI tool implementation

- Fuzzy directory matching (FZF style)
- Bookmark management (add/rm/list)
- Jump history tracking
- Shell plugin for zsh/bash"
```

---

## 总结

完成以上 13 个任务后，你将拥有一个功能完整的 `ccd` 目录跳转工具，具备：

- ✅ FZF 风格的模糊匹配
- ✅ 书签管理（增删查）
- ✅ 访问历史记录
- ✅ Shell 集成插件

**使用方式：**

```bash
# 编译安装
cargo build --release
cp target/release/ccd ~/.local/bin/  # 或使用 cargo install

# 基本用法
ccd myproj              # 跳转到匹配目录
ccd add proj             # 添加书签
ccd list                 # 列出书签
ccd hist                 # 查看历史
ccd rm proj              # 删除书签

# Shell 集成 (可选)
echo 'source /path/to/ccd/shell/ccd.sh' >> ~/.zshrc
source ~/.zshrc
ccd myproj               # 直接跳转生效
```
