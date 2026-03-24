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
