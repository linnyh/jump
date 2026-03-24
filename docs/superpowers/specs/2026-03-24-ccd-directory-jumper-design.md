# ccd - 目录跳转 CLI 工具设计文档

**版本**: 1.0
**日期**: 2026-03-24
**状态**: 已批准

---

## 1. 概述

`ccd` 是一个面向 macOS/Linux 用户的命令行工具，用于快速跳转到常用目录。它结合了书签管理和访问历史，通过 FZF 风格的模糊匹配让用户能够用最短的输入到达目标目录。

### 1.1 核心价值

- **速度**：比 `cd` 更快到达目标目录
- **智能**：FZF 风格模糊匹配，无需精确输入
- **简洁**：一个命令，多种子命令，统一体验

---

## 2. 功能规格

### 2.1 命令列表

| 命令 | 说明 | 示例 |
|------|------|------|
| `ccd <pattern>` | 模糊匹配并跳转 | `ccd myproj` |
| `ccd add <name>` | 添加书签 | `ccd add proj` |
| `ccd rm <name>` | 删除书签 | `ccd rm proj` |
| `ccd list` | 列出所有书签 | `ccd list` |
| `ccd hist` | 列出最近访问目录 | `ccd hist` |
| `ccd -i` | 交互式选择模式 | `ccd -i` |
| `ccd -e` | 打开配置文件 | `ccd -e` |

### 2.2 跳转逻辑

1. **短词直接跳转**：当输入简短且匹配结果明确时，直接跳转到得分最高的目录
2. **模糊时进入交互**：当匹配得分相近或需要选择时，调用交互式选择器

### 2.3 模糊匹配算法

采用 FZF 风格评分机制：

| 匹配类型 | 优先级 | 说明 |
|----------|--------|------|
| 连续匹配 | 最高 | 输入字符在路径中连续出现 |
| 首字母匹配 | 高 | 输入为首字母序列（如 `sp` → `src/project`） |
| 分散匹配 | 中 | 输入字符分散在路径中 |
| 频率加成 | 低 | 同分时，访问频率高的优先 |

---

## 3. 数据存储

### 3.1 存储位置

```
~/.config/ccd/
├── bookmarks.json   # 书签数据
└── history.json     # 访问历史
```

### 3.2 bookmarks.json 结构

```json
{
  "bookmarks": {
    "myproj": "/Users/name/projects/myproject",
    "docs": "/Users/name/Documents",
    "dl": "/Users/name/Downloads"
  }
}
```

### 3.3 history.json 结构

```json
{
  "entries": [
    {
      "path": "/Users/name/projects/abc",
      "access_count": 5,
      "last_access": "2025-01-15T10:30:00Z"
    },
    {
      "path": "/Users/name/Downloads",
      "access_count": 12,
      "last_access": "2025-01-15T09:15:00Z"
    }
  ]
}
```

### 3.4 数据完整性

- 启动时检查 JSON 文件是否存在，不存在则创建空文件
- 写入前备份旧文件（`*.json.bak`）
- JSON 解析失败时，报告错误并创建新文件

---

## 4. 项目结构

```
ccd/
├── Cargo.toml
├── src/
│   ├── main.rs           # 入口，CLI 参数解析
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── jump.rs        # 跳转逻辑
│   │   ├── add.rs         # 添加书签
│   │   ├── rm.rs          # 删除书签
│   │   ├── list.rs        # 列出书签
│   │   └── hist.rs        # 历史记录
│   ├── core/
│   │   ├── mod.rs
│   │   ├── storage.rs     # JSON 读写
│   │   ├── matcher.rs     # 模糊匹配算法
│   │   └── jumper.rs      # 跳转执行
│   └── config.rs          # 路径配置
└── shell/
    └── ccd.sh             # 可选 shell 插件
```

### 4.1 模块职责

| 模块 | 职责 |
|------|------|
| `main.rs` | CLI 入口，命令路由 |
| `commands/*` | 各子命令实现 |
| `core/storage.rs` | 数据持久化 |
| `core/matcher.rs` | 模糊匹配评分 |
| `core/jumper.rs` | 目录切换执行 |
| `config.rs` | 配置路径管理 |

---

## 5. Shell 集成

### 5.1 独立使用（无需配置）

```bash
ccd myproj          # 直接跳转
ccd -i              # 交互选择
```

### 5.2 Shell 插件（可选）

在 `.zshrc` 或 `.bashrc` 中添加：

```bash
source /path/to/ccd/shell/ccd.sh
```

提供增强功能：
- `c` 命令：等同于 `ccd`
- Tab 补全支持

---

## 6. 错误处理

| 场景 | 处理方式 | 退出码 |
|------|----------|--------|
| 无匹配结果 | 提示 "No matching directory found"，不切换目录 | 1 |
| 多个高匹配结果 | 提示模糊选择，进入交互模式 | 0 |
| 目录不存在 | 提示错误信息 | 1 |
| JSON 损坏 | 备份旧文件，创建新文件，继续执行 | 0 |
| 无权限写入 | 提示错误信息 | 1 |

---

## 7. 技术选型

| 组件 | 选择 | 理由 |
|------|------|------|
| 编程语言 | Rust | 性能最佳，二进制分发简单，类型安全 |
| CLI 框架 | Clap | Rust 生态最成熟，使用广泛 |
| JSON 处理 | serde_json | 官方推荐，性能好 |
| 模糊匹配 | 自实现 | 轻量可控，参考 fzf 算法 |

---

## 8. 测试策略

### 8.1 单元测试

- 模糊匹配算法评分逻辑
- JSON 序列化/反序列化
- 路径处理函数

### 8.2 集成测试

- CLI 命令端到端测试
- 文件系统操作测试

### 8.3 手动测试

- 实际终端使用验证

---

## 9. 分发方式

- **源码编译**：通过 `cargo install ccd` 安装
- **预编译二进制**：提供各平台的压缩包下载
- **Shell 插件**：独立文件，用户按需 source

---

## 10. 里程碑

1. **M1**: 核心跳转功能 + 模糊匹配
2. **M2**: 书签管理（增删改查）
3. **M3**: 访问历史记录
4. **M4**: 交互式选择器
5. **M5**: Shell 插件
6. **M6**: 文档 + 发布
