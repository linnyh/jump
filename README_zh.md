<div align="center">

# 🚀 Jump - Lightning Fast Directory Jumper

<img src="img/icon.png" width="128" height="128" style="border-radius: 20%;">

[![Version](https://img.shields.io/badge/version-0.1.1-blue.svg)](https://github.com/linnyh/jump)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.56+-orange.svg)](https://www.rust-lang.org)

<p><b>Directory jumping at the speed of light!</b></p>
<p>jump is a lightweight CLI tool that combines smart bookmark management with fuzzy matching to make directory navigation 10x faster.</p>

</div>

---

## Features

| Feature | Description |
|:---|:---|
| 🎯 **CD Replacement** | Full `cd` replacement with `..`, `/path`, `-` and more |
| 🔍 **Fuzzy Matching** | Smart fuzzy search like fzf, jump as you type |
| 🔖 **Bookmarks** | Save frequently used directories with group support |
| 📜 **Session History** | Auto-records directories visited in current session |
| ↩️ **Back Function** | `j --back` returns to previous jump location |
| 🌳 **Project Roots** | `j -R` to jump to project roots |
| 🔢 **Tab Completion** | Smart completion for bookmarks and local directories |
| 🌈 **Cross Platform** | Works on macOS and Linux |

---

## Installation

### Option 1: Pre-built Binary (Recommended)

```bash
curl -L https://github.com/linnyh/jump/releases/download/v0.1.1/j -o /usr/local/bin/j
chmod +x /usr/local/bin/j

curl -L https://github.com/linnyh/jump/releases/download/v0.1.1/j.sh -o /usr/local/bin/j.sh
echo 'source /usr/local/bin/j.sh' >> ~/.zshrc
source ~/.zshrc
```

### Option 2: Homebrew

```bash
brew install linnyh/tap/jump
echo 'source $(brew --prefix)/opt/jump/share/jump/j.sh' >> ~/.zshrc
source ~/.zshrc
```

### Option 3: Build from Source

```bash
git clone https://github.com/linnyh/jump.git
cd jump
cargo install --locked --path .
echo 'source /path/to/jump/shell/j.sh' >> ~/.zshrc
source ~/.zshrc
```

---

## Quick Start

```bash
j ..                 # Jump to parent directory
j /path              # Jump to absolute path
j -                  # Jump to previous directory
j myapp              # Fuzzy jump to bookmark or directory
j -R                 # List all project roots
j -R myapp           # Jump to project root
j -a myapp           # Add bookmark
j -l                 # List all bookmarks
```

---

## Usage

### 📁 Directory Jumping (cd replacement)

```bash
j ..                 # Jump to parent directory
j ../config          # Jump to relative path
j /Users/admin       # Jump to absolute path
j -                  # Jump to previous directory (same as cd -)
j --back             # Jump back to previous j location
j -b                 # Short for --back
```

### 🔖 Bookmark Management

```bash
# Add bookmark
cd ~/Projects/myapp
j -a myapp                    # Add bookmark named myapp
j -a work --group personal    # Add to personal group

# List bookmarks
j -l                         # List all bookmarks
j -l --group personal        # List bookmarks in personal group
j -g                         # List all groups

# Remove bookmark
j -d myapp                   # Remove bookmark named myapp
```

### 🌳 Project Roots

```bash
j -R                 # List all project roots found from current directory
j -R myapp           # Fuzzy match and jump to project root
```

Supported markers: `.git`, `Cargo.toml`, `package.json`, `go.mod`, `pyproject.toml`, `pom.xml`, `build.gradle`, `CMakeLists.txt`, `Makefile`, etc.

### 🔍 Fuzzy Jump

```bash
j myapp              # Fuzzy match bookmark names
j proj               # Auto-match most relevant directory
j doc                # Match docs subdirectory in current directory
j                    # Show session history (no args)
```

### 📋 Other Commands

```bash
j -H                 # Show jump history (sorted by frequency)
j -r                 # Show session history
j -i                 # Interactive selection (uses fzf if available)
j -e                 # Open config file in editor
j --help             # Show help
```

---

## How It Works

### Matching Priority

When you run `j proj`, j matches in this order:

```
1️⃣ Bookmark name match (weight x2 + name prefix +500 bonus)
   ↓ If "proj" or "project" bookmark found
2️⃣ Bookmark path match
   ↓ Search for "proj" in bookmark paths
3️⃣ Local directory match
   ↓ Search for "proj" subdirectory in current dir
4️⃣ Session history match
   ↓ Search for "proj" in session history
5️⃣ Project root match
   ↓ Search upward for .git, Cargo.toml, etc.
```

### Session History vs Bookmarks

| Type | Description | Persistence |
|:---|:---|:---|
| **Bookmark** | Manually added fixed directories | Persisted |
| **Session History** | Auto-recorded visit history | Current session only |

---

## Configuration

### Config Location

| OS | Path |
|:---|:---|
| macOS | `~/Library/Application Support/jump/` |
| Linux | `~/.config/jump/` |

### Config Files

```
jump/
├── bookmarks.json    # Bookmark data
└── history.json      # Jump history
```

### Edit Config Directly

```bash
# Method 1: Open in editor
j -e

# Method 2: Edit directly
vim ~/Library/Application\ Support/jump/bookmarks.json
```

### bookmarks.json Example

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

## FAQ

### Q: What's the difference between `j --back` and `cd -`?

**`cd -`** returns to the previous directory (based on shell directory stack).

**`j --back`** returns to the previous j jump location.

```
Current: /home/user/project
Run: j work        → Jump to /home/user/work
Run: j --back       → Return to /home/user/project (j's previous location)
Run: cd ..          → Jump to /home/user
Run: cd -           → Return to /home/user/project (cd's directory stack)
Run: j --back       → Still returns to /home/user/work (j's jump record)
```

### Q: What special rules apply to bookmark name matching?

Bookmark name prefix matching gets a +500 bonus, far exceeding any path match score.

```
Example: Bookmark "ProjectThinking" -> /path/to/ProjectThinking
Input "proj" matches "ProjectThinking" with +500 bonus
Input "pt" also matches due to fuzzy matching rules
```

### Q: How does fuzzy matching work?

j uses fuzzy-matcher algorithm:

```
Input "myapp" can match:
- myapp          ✓ Exact match
- my-app         ✓ Fuzzy match
- m_y_a_p_p      ✓ Acronym match
- mp             ✓ Subsequence match
```

### Q: How does interactive selection work?

j prefers fzf for interactive selection. If fzf is not installed, it falls back to numbered selector:

```bash
j -i                 # Interactive selection
```

---

## Dependencies

| Dependency | Version | Description |
|:---|:---|:---|
| Rust | 1.56+ | Build environment |
| fzf | Latest | Interactive selection (optional) |

---

## License

MIT License - Use freely 🚀

---

<div align="center">

**Made with ❤️ using Rust**

</div>
