use std::path::PathBuf;

pub struct Config {
    config_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("jump");
        Self { config_dir }
    }

    #[cfg(test)]
    pub fn from_path(path: &std::path::Path) -> Self {
        Self {
            config_dir: path.to_path_buf(),
        }
    }

    #[cfg(test)]
    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn bookmarks_path(&self) -> PathBuf {
        self.config_dir.join("bookmarks.json")
    }

    pub fn history_path(&self) -> PathBuf {
        self.config_dir.join("history.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir() {
        let config = Config::new();
        assert!(config.config_dir().ends_with("jump"));
    }

    #[test]
    fn test_bookmarks_path() {
        let config = Config::new();
        assert!(config.bookmarks_path().ends_with("bookmarks.json"));
    }

    #[test]
    fn test_history_path() {
        let config = Config::new();
        assert!(config.history_path().ends_with("history.json"));
    }
}
