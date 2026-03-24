use crate::core::storage;
use crate::Config;

pub struct AddCommand {
    pub name: String,
}

impl AddCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let current_path = current_dir.to_string_lossy().to_string();
        let mut bookmarks = storage::load_bookmarks(config)?;
        bookmarks.insert(self.name.clone(), current_path.clone());
        storage::save_bookmarks(config, &bookmarks)?;
        println!("Added bookmark '{}' -> {}", self.name, current_path);
        Ok(())
    }
}
