# j - 快速目录跳转工具

一个轻量级的命令行工具，用于快速跳转到常用目录。结合书签管理和模糊匹配，让目录跳转变得高效便捷。

## 功能特性

- **模糊匹配**: FZF 风格的模糊搜索，输入即跳转
- **书签管理**: 保存常用目录，方便快速访问
- **分组管理**: 支持书签按项目/用途分类
- **会话历史**: 自动记录当前会话访问过的目录
- **本地优先**: 优先匹配当前目录下的子目录
- **跨平台**: 支持 macOS 和 Linux

## 安装

### 从源码安装

```bash
cargo install --path .
```

### Shell 插件（可选）

在 `~/.zshrc` 或 `~/.bashrc` 中添加：

```bash
source /path/to/j/shell/j.sh
```

## 使用方法

### 基本命令

```bash
j <pattern>          # 模糊匹配并跳转
j ~                  # 跳转到主目录
j                    # 查看会话历史
```

### 书签管理

```bash
j add <name>                     # 添加书签（当前目录）
j add <name> --group <group>     # 添加书签到指定分组
j rm <name>                      # 删除书签
j list                           # 列出所有书签
j list --group <group>           # 列出指定分组的书签
j groups                         # 列出所有分组
```

### 其他命令

```bash
j hist               # 查看跳转历史
j -i                 # 交互式选择（需安装 fzf）
j -e                 # 编辑配置文件
j <pattern>          # 模糊匹配跳转
```

## 匹配优先级

1. **书签名称匹配**（最高优先级）
2. **书签路径匹配**
3. **本地目录匹配**（当前目录下的子目录）
4. **会话历史匹配**

## 配置

配置文件位于 `~/.config/ccd/` 目录：
- `bookmarks.json` - 书签数据
- `history.json` - 跳转历史

## 示例

```bash
# 添加书签
cd ~/Projects/myapp
j add app

# 添加带分组的书签
cd ~/Projects/work
j add project --group work
j add doc --group work

cd ~/Documents
j add notes --group personal

# 列出所有书签（按分组显示）
j list

# 查看所有分组
j groups

# 只查看 work 分组的书签
j list --group work

# 快速跳转
j app                # 跳转到 ~/Projects/myapp
j doc                # 跳转到本地 docs 目录
j work               # 跳转到 work 分组中匹配的书签
j                    # 查看会话历史
```

## 依赖

- [fzf](https://github.com/junegunn/fzf)（交互式选择功能可选）
- Rust 1.56+

## License

MIT
