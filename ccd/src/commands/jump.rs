use crate::core::matcher;
use crate::Config;

pub struct JumpCommand {
    pub pattern: Option<String>,
}

impl JumpCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let Some(pattern) = &self.pattern else {
            return Err("Usage: j <pattern> - jump to a directory".to_string());
        };

        let bookmarks = crate::core::storage::load_bookmarks(config)?;
        let candidates: Vec<&str> = bookmarks.values().into_iter().map(|s| s.as_str()).collect();
        let matches = matcher::fuzzy_match(pattern, &candidates);

        if matches.is_empty() {
            return Err("No matching directory found".to_string());
        }

        let best = matches.first().ok_or("No match")?;
        println!("{}", crate::core::jumper::generate_cd_script(&best.path));
        Ok(())
    }
}
