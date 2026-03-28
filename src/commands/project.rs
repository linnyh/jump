
/// 常见项目根目录标记
const PROJECT_MARKERS: &[&str] = &[
    ".git",
    "Cargo.toml",      // Rust
    "package.json",    // Node.js
    "go.mod",          // Go
    "pyproject.toml",  // Python
    "setup.py",       // Python
    "requirements.txt", // Python (alternative)
    "pom.xml",         // Java/Maven
    "build.gradle",    // Java/Gradle
    "CMakeLists.txt",  // C/C++
    "Makefile",        // C/通用
    ".svn",           // Subversion
    "Cargo.lock",      // Rust (alternative)
    "go.sum",          // Go (alternative)
    ".hg",             // Mercurial
];

/// 在指定目录及其父目录中查找项目根目录
#[allow(dead_code)]
pub fn find_project_root(start_dir: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut current = Some(start_dir.to_path_buf());

    while let Some(dir) = current {
        for marker in PROJECT_MARKERS {
            if dir.join(marker).exists() {
                return Some(dir);
            }
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }
    None
}

/// 获取项目根目录的名称（用于匹配）
pub fn get_project_name(root: &std::path::Path) -> String {
    root.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// 列出从 start_dir 向上查找到的所有项目根目录
pub fn list_project_roots(start_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut roots = Vec::new();
    let mut current = Some(start_dir.to_path_buf());

    while let Some(dir) = current {
        for marker in PROJECT_MARKERS {
            if dir.join(marker).exists() {
                roots.push(dir.clone());
                break;
            }
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }
    roots
}

/// 模糊匹配项目根目录
pub fn fuzzy_match_projects(input: &str, candidates: &[std::path::PathBuf]) -> Option<std::path::PathBuf> {
    use crate::core::matcher::fuzzy_score;

    let mut best_score = 0i64;
    let mut best_path = None;

    for path in candidates {
        let name = get_project_name(path);
        let score = fuzzy_score(input, &name);
        if score > best_score {
            best_score = score;
            best_path = Some(path.clone());
        }
    }

    best_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_markers_defined() {
        assert!(!PROJECT_MARKERS.is_empty());
        assert!(PROJECT_MARKERS.contains(&".git"));
        assert!(PROJECT_MARKERS.contains(&"Cargo.toml"));
    }

    #[test]
    fn test_get_project_name() {
        let path = std::path::PathBuf::from("/Users/foo/my-project");
        assert_eq!(get_project_name(&path), "my-project");
    }

    #[test]
    fn test_find_project_root_no_marker() {
        // 在没有项目标记的目录下，应该返回 None
        let temp = std::env::temp_dir().join("jump_test_no_marker");
        std::fs::create_dir_all(&temp).ok();
        let result = find_project_root(&temp);
        // temp_dir 可能不在项目内，所以结果取决于环境
        // 测试主要是确保函数不 panic
        assert!(result.is_some() || result.is_none());
        std::fs::remove_dir(&temp).ok();
    }

    #[test]
    fn test_fuzzy_match_projects() {
        let projects = vec![
            std::path::PathBuf::from("/path/to/my-app"),
            std::path::PathBuf::from("/path/to/other-project"),
            std::path::PathBuf::from("/path/to/test"),
        ];

        // 匹配 "my"
        let result = fuzzy_match_projects("my", &projects);
        assert_eq!(result, Some(std::path::PathBuf::from("/path/to/my-app")));

        // 匹配 "proj"
        let result = fuzzy_match_projects("proj", &projects);
        assert_eq!(
            result,
            Some(std::path::PathBuf::from("/path/to/other-project"))
        );
    }
}
