use crate::core::storage;
use crate::Config;

pub struct RmCommand {
    pub name: String,
}

impl RmCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let mut bookmarks = storage::load_bookmarks(config)?;
        if let Some(removed) = bookmarks.remove(&self.name) {
            storage::save_bookmarks(config, &bookmarks)?;
            println!("Removed bookmark '{}' -> {}", self.name, removed);
            Ok(())
        } else {
            Err(format!("Bookmark '{}' not found", self.name))
        }
    }
}
