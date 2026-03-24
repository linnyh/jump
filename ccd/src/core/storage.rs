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
        Self { bookmarks: HashMap::new() }
    }

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
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            access_count: 0,
            last_access: chrono::Utc::now().to_rfc3339(),
        }
    }

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
        Self { entries: Vec::new() }
    }

    pub fn add_or_update(&mut self, path: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.path == path) {
            entry.increment_access();
        } else {
            self.entries.push(HistoryEntry::new(path));
        }
        self.entries.sort_by(|a, b| b.access_count.cmp(&a.access_count));
    }

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
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read bookmarks: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse bookmarks: {}", e))
}

pub fn save_bookmarks(config: &crate::config::Config, bookmarks: &Bookmarks) -> Result<(), String> {
    let path = config.bookmarks_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    let content = serde_json::to_string_pretty(bookmarks)
        .map_err(|e| format!("Failed to serialize bookmarks: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write bookmarks: {}", e))
}

pub fn load_history(config: &crate::config::Config) -> Result<History, String> {
    let path = config.history_path();
    if !path.exists() {
        return Ok(History::new());
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read history: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse history: {}", e))
}

pub fn save_history(config: &crate::config::Config, history: &History) -> Result<(), String> {
    let path = config.history_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    let content = serde_json::to_string_pretty(history)
        .map_err(|e| format!("Failed to serialize history: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write history: {}", e))
}
