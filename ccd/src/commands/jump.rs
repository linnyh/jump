use crate::core::matcher;
use crate::Config;
use std::fs;

pub struct JumpCommand {
    pub pattern: Option<String>,
    pub cwd: Option<String>,  // 从 shell 传入的工作目录
}

impl JumpCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let Some(pattern) = &self.pattern else {
            return Err("Usage: j <pattern> - jump to a directory".to_string());
        };

        // 如果是 ~ 单独，直接返回 home 目录
        if pattern == "~" {
            let home = dirs::home_dir()
                .map(|p| p.to_string_lossy().to_string())
                .ok_or("Cannot find home directory")?;
            println!("{}", crate::core::jumper::generate_cd_script(&home));
            return Ok(());
        }

        // 如果输入就是 home 目录本身（shell 已展开 ~），直接返回
        if let Ok(home) = std::env::var("HOME") {
            if *pattern == home {
                println!("{}", crate::core::jumper::generate_cd_script(&home));
                return Ok(());
            }
        }

        // 展开 ~ 前缀为实际路径
        let pattern = if pattern.starts_with("~/") {
            dirs::home_dir()
                .map(|home| format!("{}/{}", home.to_string_lossy(), &pattern[2..]))
                .unwrap_or_else(|| pattern.clone())
        } else {
            pattern.clone()
        };

        // 确定搜索目录
        let search_dir = if let Some(ref cwd) = self.cwd {
            std::path::PathBuf::from(cwd)
        } else {
            std::env::current_dir()
                .map_err(|e| format!("Cannot get current directory: {}", e))?
        };

        // 1. 查找本地目录（只匹配目录名）
        let local_match = find_best_local_match(&search_dir, &pattern);

        // 2. 加载书签并匹配
        let bookmarks = crate::core::storage::load_bookmarks(config)?;
        let bookmark_match = find_best_bookmark_match(&bookmarks, &pattern);

        // 3. 决定最终匹配
        // 如果本地目录是更精确的匹配（目录名以 pattern 开头），优先本地
        // 否则优先书签
        let final_path = if let Some((local_score, local_path)) = local_match {
            if let Some((bookmark_score, _)) = &bookmark_match {
                // 本地目录名以 pattern 开头，得分 * 1.5 优先
                let name = search_dir.join(&local_path);
                let name_str = name.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                if name_str.to_lowercase().starts_with(&pattern.to_lowercase()) {
                    // 本地目录名以 pattern 开头
                    if (local_score * 15 / 10) >= *bookmark_score {
                        Some(local_path)
                    } else {
                        bookmark_match.map(|(_, p)| p)
                    }
                } else {
                    // 书签优先
                    bookmark_match.map(|(_, p)| p)
                }
            } else {
                Some(local_path)
            }
        } else {
            bookmark_match.map(|(_, p)| p)
        };

        if let Some(path) = final_path {
            println!("{}", crate::core::jumper::generate_cd_script(&path));
            return Ok(());
        }

        // 4. 再匹配会话历史
        let history_match = crate::commands::recent::fuzzy_match_session_history(&pattern);
        if let Some(path) = history_match {
            println!("{}", crate::core::jumper::generate_cd_script(&path));
            return Ok(());
        }

        // 5. 都没匹配到，返回错误
        Err("No matching directory found".to_string())
    }
}

/// 查找最佳书签匹配，返回 (得分, 路径)
fn find_best_bookmark_match(
    bookmarks: &crate::core::storage::Bookmarks,
    pattern: &str,
) -> Option<(u32, String)> {
    let mut best_score = 0u32;
    let mut best_path = None;

    for (name, path) in &bookmarks.bookmarks {
        // 先匹配书签名称
        let name_score = matcher::fuzzy_score(pattern, name);

        // 再匹配路径
        let path_score = matcher::fuzzy_score(pattern, path);

        // 取较高分，但书签名称优先
        let score = if name_score > 0 {
            name_score * 2  // 书签名称权重更高
        } else {
            path_score
        };

        if score > best_score {
            best_score = score;
            best_path = Some(path.clone());
        }
    }

    best_path.map(|p| (best_score, p))
}

/// 查找最佳本地目录匹配，返回 (得分, 完整路径)
fn find_best_local_match(dir: &std::path::Path, pattern: &str) -> Option<(u32, String)> {
    let mut best_score = 0u32;
    let mut best_path = None;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    let score = matcher::fuzzy_score(pattern, &name_str);

                    if score > best_score {
                        best_score = score;
                        best_path = Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    best_path.map(|p| (best_score, p))
}
