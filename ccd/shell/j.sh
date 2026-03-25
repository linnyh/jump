#!/bin/bash
# j shell plugin
# 在 .zshrc 或 .bashrc 中 source 此文件

# 确保 ~/.cargo/bin 在 PATH 中
export PATH="$HOME/.cargo/bin:$PATH"

j() {
    # 对于需要终端交互的命令，直接调用原始命令
    if [[ "$1" == "-e" ]] || [[ "$1" == "--edit" ]]; then
        command j "$@"
        return $?
    fi

    # 获取当前工作目录
    local current_dir="$(pwd)"

    local result
    result=$(command j --cwd "$current_dir" "$@")

    # 如果没有输出，直接返回
    if [[ -z "$result" ]]; then
        return
    fi

    # 检查是否包含 cd 命令（需要 eval）
    # 对于 --help、-h、-V 等选项，不 eval
    if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]] || [[ "$1" == "-V" ]] || [[ "$1" == "--version" ]]; then
        echo "$result"
        return
    fi

    # 对于帮助子命令（add --help 等），打印结果
    for arg in "$@"; do
        if [[ "$arg" == "--help" ]]; then
            echo "$result"
            return
        fi
    done

    # 只有包含 cd 命令时才 eval
    if [[ "$result" == cd\ * ]]; then
        # 执行 cd 命令并记录历史
        eval "$result"
        local cd_status=$?

        if [[ $cd_status -eq 0 ]]; then
            # 记录到会话历史
            command j --record-current 2>/dev/null
        fi
    else
        echo "$result"
    fi
}

# 内部命令：记录当前目录到会话历史
_j_record() {
    command j --record-current 2>/dev/null
}

# Tab 补全 (仅 zsh，延迟加载避免兼容性问题)
_j() {
    local -a commands
    commands=(
        "add:Add bookmark for current directory"
        "rm:Remove a bookmark"
        "list:List all bookmarks"
        "hist:Show jump history"
        "recent:Show session history"
    )
    _describe 'command' commands
}

# 安全地注册补全函数
if [[ -n "$ZSH_VERSION" ]] && [[ -f /usr/share/zsh/site-functions/_j ]]; then
    compdef _j j 2>/dev/null
fi
