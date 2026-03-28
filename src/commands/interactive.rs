use crate::core::storage;
use crate::Config;

pub struct InteractiveCommand;

impl InteractiveCommand {
    pub fn execute(config: &Config) -> Result<(), String> {
        // 加载书签和历史
        let bookmarks = storage::load_bookmarks(config)?;
        let history = storage::load_history(config)?;

        // 合并候选目录（带名称标注）
        let mut candidates: Vec<(&str, Option<&str>)> = bookmarks
            .bookmarks
            .iter()
            .map(|(name, entry)| (entry.path.as_str(), Some(name.as_str())))
            .collect();

        for entry in &history.entries {
            if !candidates.iter().any(|(p, _)| *p == entry.path) {
                candidates.push((entry.path.as_str(), None));
            }
        }

        if candidates.is_empty() {
            return Err("No directories available. Add bookmarks first.".to_string());
        }

        // 优先使用 fzf，没有则使用编号选择
        let selected = if has_fzf() {
            run_fzf_selector(&candidates)
        } else {
            run_numbered_selector(&candidates)
        }?;

        match selected {
            Some(path) => {
                println!("{}", crate::core::jumper::generate_cd_script(&path));
                Ok(())
            }
            None => Ok(()),  // 用户取消，不报错
        }
    }
}

/// 检查 fzf 是否可用
fn has_fzf() -> bool {
    std::process::Command::new("fzf")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// 运行 fzf 进行交互式选择
fn run_fzf_selector(candidates: &[(&str, Option<&str>)]) -> Result<Option<String>, String> {
    use std::io::Write;
    use std::process::{Command as ProcCommand, Stdio};

    // 构建 fzf 输入（显示名称和路径）
    let input: String = candidates
        .iter()
        .map(|(path, name)| {
            if let Some(n) = name {
                format!("{}\t{}", n, path)
            } else {
                path.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut child = ProcCommand::new("fzf")
        .args([
            "--height=50%",
            "--layout=reverse",
            "--preview-window=right:50%",
            "--with-nth=2..",  // 只在搜索中使用路径部分
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start fzf: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .map_err(|e| format!("Failed to write to fzf: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for fzf: {}", e))?;

    if output.status.success() {
        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if raw.is_empty() {
            Ok(None)
        } else {
            // 提取路径部分（tab 后面的内容）
            if let Some(pos) = raw.rfind('\t') {
                Ok(Some(raw[pos + 1..].to_string()))
            } else {
                Ok(Some(raw))
            }
        }
    } else {
        Ok(None)
    }
}

/// 编号选择器（无 fzf 时的 fallback）
fn run_numbered_selector(candidates: &[(&str, Option<&str>)]) -> Result<Option<String>, String> {
    use std::io::{self, Write};

    println!("\n  Select directory (press number or Ctrl+C to cancel):\n");

    for (i, (path, name)) in candidates.iter().enumerate() {
        if let Some(n) = name {
            println!("  {:>3}. [{}] {}", i + 1, n, path);
        } else {
            println!("  {:>3}. {}", i + 1, path);
        }
    }
    print!("\n  > ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Ok(None);
    }

    let choice = input.trim();
    match choice.parse::<usize>() {
        Ok(n) if n > 0 && n <= candidates.len() => Ok(Some(candidates[n - 1].0.to_string())),
        _ => Ok(None),
    }
}
