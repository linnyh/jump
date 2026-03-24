use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmarks {
    #[serde(flatten)]
    pub bookmarks: HashMap<String, String>,
}

impl Bookmarks {
    pub fn new() -> Self {
        Self {
            bookmarks: HashMap::new(),
        }
    }

    #[cfg(test)]
    pub fn get(&self, name: &str) -> Option<&String> {
        self.bookmarks.get(name)
    }

    pub fn insert(&mut self, name: String, path: String) {
        self.bookmarks.insert(name, path);
    }

    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.bookmarks.remove(name)
    }

    pub fn values(&self) -> Vec<&String> {
        self.bookmarks.values().collect()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.bookmarks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bookmarks.is_empty()
    }
}

impl Default for Bookmarks {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub path: String,
    pub access_count: u32,
    #[serde(rename = "last_access")]
    pub last_access: String,
}

impl HistoryEntry {
    #[cfg(test)]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            access_count: 0,
            last_access: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[cfg(test)]
    pub fn increment_access(&mut self) {
        self.access_count += 1;
        self.last_access = chrono::Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn add_or_update(&mut self, path: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.path == path) {
            entry.increment_access();
        } else {
            let mut entry = HistoryEntry::new(path);
            entry.increment_access(); // 首次访问计为1次
            self.entries.push(entry);
        }
        self.entries
            .sort_by(|a, b| b.access_count.cmp(&a.access_count));
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn recent(&self, n: usize) -> Vec<&HistoryEntry> {
        self.entries.iter().take(n).collect()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

// 注意：load_bookmarks, save_bookmarks, load_history, save_history 函数将在 Task 11 中实现

pub fn load_bookmarks(config: &crate::config::Config) -> Result<Bookmarks, String> {
    let path = config.bookmarks_path();
    if !path.exists() {
        return Ok(Bookmarks::new());
    }
    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read bookmarks: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse bookmarks: {}", e))
}

pub fn save_bookmarks(config: &crate::config::Config, bookmarks: &Bookmarks) -> Result<(), String> {
    let path = config.bookmarks_path();

    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 备份旧文件
    if path.exists() {
        let backup_path = format!("{}.bak", path.display());
        let _ = fs::copy(&path, &backup_path);
    }

    let content = serde_json::to_string_pretty(bookmarks)
        .map_err(|e| format!("Failed to serialize bookmarks: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write bookmarks: {}", e))
}

pub fn load_history(config: &crate::config::Config) -> Result<History, String> {
    let path = config.history_path();
    if !path.exists() {
        return Ok(History::new());
    }
    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read history: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse history: {}", e))
}

#[allow(dead_code)]
pub fn save_history(config: &crate::config::Config, history: &History) -> Result<(), String> {
    let path = config.history_path();

    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 备份旧文件
    if path.exists() {
        let backup_path = format!("{}.bak", path.display());
        let _ = fs::copy(&path, &backup_path);
    }

    let content = serde_json::to_string_pretty(history)
        .map_err(|e| format!("Failed to serialize history: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write history: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    fn temp_config() -> Config {
        let temp_dir = env::temp_dir().join(format!("ccd_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);
        Config::from_path(&temp_dir)
    }

    #[test]
    fn test_load_nonexistent_file() {
        let config = temp_config();
        // 应该返回空 Bookmarks 而不报错
        let result = load_bookmarks(&config);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_load_nonexistent_history() {
        let config = temp_config();
        // 应该返回空 History 而不报错
        let result = load_history(&config);
        assert!(result.is_ok());
        assert!(result.unwrap().entries.is_empty());
    }

    #[test]
    fn test_save_and_load_bookmarks() {
        let config = temp_config();
        let mut bookmarks = Bookmarks::new();
        bookmarks.insert("home".to_string(), "/Users/test".to_string());
        bookmarks.insert("work".to_string(), "/Users/test/work".to_string());

        // 保存
        let save_result = save_bookmarks(&config, &bookmarks);
        assert!(save_result.is_ok());

        // 加载
        let loaded = load_bookmarks(&config).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.get("home"), Some(&"/Users/test".to_string()));
        assert_eq!(loaded.get("work"), Some(&"/Users/test/work".to_string()));
    }

    #[test]
    fn test_save_and_load_history() {
        let config = temp_config();
        let mut history = History::new();
        history.add_or_update("/Users/test/dir1");
        history.add_or_update("/Users/test/dir2");
        history.add_or_update("/Users/test/dir1"); // 重复访问

        // 保存
        let save_result = save_history(&config, &history);
        assert!(save_result.is_ok());

        // 加载
        let loaded = load_history(&config).unwrap();
        assert_eq!(loaded.entries.len(), 2);

        // dir1 有2次访问，应该排第一
        let dir1_entry = loaded
            .entries
            .iter()
            .find(|e| e.path == "/Users/test/dir1")
            .unwrap();
        assert_eq!(dir1_entry.access_count, 2);

        // 验证排序：dir1 应该在第一位
        assert_eq!(loaded.entries[0].path, "/Users/test/dir1");
    }

    #[test]
    fn test_backup_on_save() {
        let config = temp_config();
        let mut bookmarks = Bookmarks::new();
        bookmarks.insert("test".to_string(), "/path".to_string());

        // 第一次保存
        save_bookmarks(&config, &bookmarks).unwrap();

        // 修改书签
        bookmarks.insert("test".to_string(), "/new_path".to_string());

        // 第二次保存，应该创建备份
        save_bookmarks(&config, &bookmarks).unwrap();

        // 验证备份文件存在
        let backup_path = format!("{}.bak", config.bookmarks_path().display());
        assert!(PathBuf::from(&backup_path).exists());
    }
}
