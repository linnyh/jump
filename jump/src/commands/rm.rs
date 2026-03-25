use crate::core::storage;
use crate::Config;

pub struct RmCommand {
    pub name: String,
}

impl RmCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let mut bookmarks = storage::load_bookmarks(config)?;

        // 先尝试直接匹配
        if let Some(removed) = bookmarks.remove(&self.name) {
            storage::save_bookmarks(config, &bookmarks)?;
            println!("Removed bookmark '{}' -> {}", self.name, removed.path);
            return Ok(());
        }

        // 如果包含 /，尝试查找匹配的书签
        if self.name.contains('/') {
            let parts: Vec<&str> = self.name.split('/').collect();
            if parts.len() == 2 {
                let (group, name) = (parts[0], parts[1]);
                let full_name = format!("{}/{}", group, name);

                if let Some(removed) = bookmarks.remove(&full_name) {
                    storage::save_bookmarks(config, &bookmarks)?;
                    println!("Removed bookmark '{}' (group: {}) -> {}", name, group, removed.path);
                    return Ok(());
                }
            }
        }

        Err(format!("Bookmark '{}' not found", self.name))
    }
}
