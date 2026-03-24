use crate::core::matcher;
use std::fs;
use std::path::PathBuf;

/// 获取会话历史文件路径
fn session_history_path() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir.join("j_session_history")
}

/// 加载会话历史
fn load_session_history() -> Vec<String> {
    let path = session_history_path();
    if !path.exists() {
        return Vec::new();
    }

    fs::read_to_string(&path)
        .map(|content| {
            content
                .lines()
                .filter(|line| !line.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// 保存会话历史
fn save_session_history(history: &[String]) {
    let path = session_history_path();
    let content = history.join("\n");
    let _ = fs::write(&path, content);
}

/// 添加到会话历史
pub fn add_to_history(path: &str) {
    let mut history = load_session_history();

    // 移除已存在的相同路径（会重新添加到前面）
    history.retain(|p| p != path);

    // 添加到最前面
    history.insert(0, path.to_string());

    // 限制历史数量
    history.truncate(50);

    save_session_history(&history);
}

/// 打印会话历史
pub fn print_session_history() {
    let history = load_session_history();
    if history.is_empty() {
        println!("No session history yet. Use 'j <pattern>' to build history.");
        return;
    }

    for (i, path) in history.iter().enumerate() {
        println!("{}: {}", i + 1, path);
    }
}

/// 从会话历史中匹配并选择
pub fn fuzzy_match_session_history(input: &str) -> Option<String> {
    let history = load_session_history();
    if history.is_empty() {
        return None;
    }

    let candidates: Vec<&str> = history.iter().map(|s| s.as_str()).collect();
    let matches = matcher::fuzzy_match(input, &candidates);

    // 如果只有一个明确匹配，直接返回
    if matches.len() == 1 {
        return Some(matches[0].path.clone());
    }

    // 如果有多个匹配，使用 fzf 选择
    if matches.len() > 1 {
        if let Some(selected) = run_fzf_selector(&matches) {
            return Some(selected);
        }
    }

    None
}

/// 运行 fzf 选择
fn run_fzf_selector(matches: &[matcher::MatchResult]) -> Option<String> {
    use std::process::{Command as ProcCommand, Stdio};
    use std::io::Write;

    let input: String = matches.iter().map(|m| m.path.as_str()).collect::<Vec<_>>().join("\n");

    let mut child = ProcCommand::new("fzf")
        .args(["--height=50%", "--layout=reverse"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(input.as_bytes());
    }

    let output = child.wait_with_output().ok()?;

    if output.status.success() {
        let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !selected.is_empty() {
            return Some(selected);
        }
    }

    None
}
