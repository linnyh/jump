use crate::core::storage;
use crate::Config;

pub struct ListCommand;

impl ListCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let bookmarks = storage::load_bookmarks(config)?;
        if bookmarks.is_empty() {
            println!("No bookmarks yet. Use 'j add <name>' to add one.");
            return Ok(());
        }
        for (name, path) in &bookmarks.bookmarks {
            println!("{} -> {}", name, path);
        }
        Ok(())
    }
}
