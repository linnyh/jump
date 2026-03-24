use crate::core::storage;
use crate::Config;

pub struct HistCommand;

impl HistCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let history = storage::load_history(config)?;
        if history.entries.is_empty() {
            println!("No history yet. Use 'ccd <pattern>' to jump and build history.");
            return Ok(());
        }
        for entry in history.entries.iter().take(20) {
            println!("{} [{}]: {}", entry.access_count, entry.last_access, entry.path);
        }
        Ok(())
    }
}
