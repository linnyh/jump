#!/bin/bash
# j shell plugin
# 在 .zshrc 或 .bashrc 中 source 此文件

# 确保 ~/.cargo/bin 在 PATH 中
export PATH="$HOME/.cargo/bin:$PATH"

# 记录上一次跳转的目录（用于 j --back）
_J_LAST_JUMP_DIR=""

j() {
    # 对于需要终端交互的命令，直接调用原始命令
    if [[ "$1" == "-e" ]] || [[ "$1" == "--edit" ]]; then
        command j "$@"
        return $?
    fi

    # 处理 j --back / j -b：返回上一次跳转的目录
    if [[ "$1" == "--back" ]] || [[ "$1" == "-b" ]]; then
        if [[ -n "$_J_LAST_JUMP_DIR" ]]; then
            cd "$_J_LAST_JUMP_DIR"
            return $?
        else
            echo "No previous jump directory" >&2
            return 1
        fi
    fi

    # 处理 cd 风格的路径（直接替换 cd）
    local arg="$1"
    case "$arg" in
        ..|.|\.\.|\.\.)
            # j . 或 j .. 直接替换为 cd
            eval "cd $arg"
            return $?
            ;;
        ../*)
            # j ../something -> cd ../something
            eval "cd $arg"
            return $?
            ;;
        ./)
            # j ./something -> cd ./something
            eval "cd $arg"
            return $?
            ;;
        /*)
            # 绝对路径 j /path -> cd /path
            eval "cd $arg"
            return $?
            ;;
        -)
            # j - 返回上一个目录（与 cd - 相同）
            cd -
            return $?
            ;;
    esac

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
        # 提取目标目录并保存当前位置
        local target_dir="${result:3}"
        target_dir="${target_dir//\'/}"
        target_dir="${target_dir//\"/}"

        # 执行 cd 命令并记录历史
        eval "$result"
        local cd_status=$?

        if [[ $cd_status -eq 0 ]]; then
            # 记录上一次跳转的目录
            _J_LAST_JUMP_DIR="$current_dir"
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

# 获取书签列表（供补全用）
_j_get_bookmarks() {
    local config_dir="${HOME}/Library/Application Support/jump"
    [[ -f "${config_dir}/bookmarks.json" ]] || config_dir="${HOME}/.config/jump"
    [[ -f "${config_dir}/bookmarks.json" ]] || return

    # 提取书签名称（JSON 中的 key）
    grep -o '"[^"]*"\s*:' "${config_dir}/bookmarks.json" 2>/dev/null | tr -d '":' | tr -d ' '
}

# Tab 补全
_j() {
    local -a commands bookmarks
    commands=(
        "add:Add bookmark for current directory"
        "rm:Remove a bookmark"
        "list:List all bookmarks"
        "groups:List all bookmark groups"
        "hist:Show jump history"
        "recent:Show session history"
    )

    local curcontext="$curcontext" state line
    typeset -A opt_args

    # 解析已输入的内容
    _arguments -C \
        '(-i --interactive)'{-i,--interactive}'[interactive selection]' \
        '(-e --edit)'{-e,--edit}'[open config file]' \
        '(-r --recent)'{-r,--recent}'[session history mode]' \
        '(-b --back)'{-b,--back}'[return to previous jump]' \
        '1: :->command' \
        '*: :->args'

    case $state in
        command)
            # 第一个参数：子命令或书签名
            bookmarks=($(_j_get_bookmarks))
            _describe 'command' commands && _describe 'bookmark' bookmarks
            ;;
        args)
            case $words[1] in
                add|rm)
                    _message "bookmark name"
                    ;;
                list)
                    _arguments '(-g --group)'{-g,--group}'[filter by group]'
                    ;;
                *)
                    ;;
            esac
            ;;
    esac
}

# 注册补全函数
compdef _j j 2>/dev/null
