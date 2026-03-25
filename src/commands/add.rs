use crate::core::storage;
use crate::Config;

pub struct AddCommand {
    pub name: String,
    pub group: Option<String>,
}

impl AddCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let current_path = current_dir.to_string_lossy().to_string();
        let mut bookmarks = storage::load_bookmarks(config)?;

        // 如果指定了分组，只打印分组信息，不添加到名称中
        if let Some(ref group) = self.group {
            println!("Added bookmark '{}' (group: {}) -> {}", self.name, group, current_path);
        } else {
            println!("Added bookmark '{}' -> {}", self.name, current_path);
        };

        bookmarks.insert(
            self.name.clone(),
            current_path,
            self.group.clone(),
        );
        storage::save_bookmarks(config, &bookmarks)?;
        Ok(())
    }
}
