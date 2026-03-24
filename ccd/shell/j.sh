#!/bin/bash
# j shell plugin
# 在 .zshrc 或 .bashrc 中 source 此文件

j() {
    local result
    result=$(command j "$@")

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
        eval "$result"
    else
        echo "$result"
    fi
}

# Tab 补全 (仅 zsh，延迟加载避免兼容性问题)
_j() {
    local -a commands
    commands=(
        "add:Add bookmark for current directory"
        "rm:Remove a bookmark"
        "list:List all bookmarks"
        "hist:Show jump history"
    )
    _describe 'command' commands
}

# 安全地注册补全函数
if [[ -n "$ZSH_VERSION" ]] && [[ -f /usr/share/zsh/site-functions/_j ]]; then
    compdef _j j 2>/dev/null
fi
