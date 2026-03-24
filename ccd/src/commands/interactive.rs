use crate::core::storage;
use crate::Config;

pub struct InteractiveCommand;

impl InteractiveCommand {
    pub fn execute(config: &Config) -> Result<(), String> {
        // 加载书签和历史
        let bookmarks = storage::load_bookmarks(config)?;
        let history = storage::load_history(config)?;

        // 合并候选目录
        let mut candidates: Vec<&str> = bookmarks.values().iter().map(|s| s.as_str()).collect();
        for entry in &history.entries {
            if !candidates.contains(&entry.path.as_str()) {
                candidates.push(&entry.path);
            }
        }

        if candidates.is_empty() {
            return Err("No directories available. Add bookmarks first.".to_string());
        }

        // 使用 fzf 选择
        let selected = run_fzf_selector(&candidates)?;

        if let Some(path) = selected {
            println!("{}", crate::core::jumper::generate_cd_script(&path));
            Ok(())
        } else {
            Err("No selection".to_string())
        }
    }
}

/// 运行 fzf 进行交互式选择
fn run_fzf_selector(candidates: &[&str]) -> Result<Option<String>, String> {
    use std::io::Write;
    use std::process::{Command as ProcCommand, Stdio};

    // 构建 fzf 输入
    let input: String = candidates.join("\n");

    // 尝试调用 fzf
    let mut child = ProcCommand::new("fzf")
        .args([
            "--height=50%",
            "--layout=reverse",
            "--preview-window=right:50%",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "fzf not found. Please install fzf: brew install fzf")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .map_err(|e| format!("Failed to write to fzf: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for fzf: {}", e))?;

    if output.status.success() {
        let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if selected.is_empty() {
            Ok(None)
        } else {
            Ok(Some(selected))
        }
    } else {
        Ok(None) // 用户取消
    }
}
