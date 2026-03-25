use crate::core::storage;
use crate::Config;

pub struct ListCommand {
    pub group: Option<String>,
}

impl ListCommand {
    pub fn execute(&self, config: &Config) -> Result<(), String> {
        let bookmarks = storage::load_bookmarks(config)?;

        if bookmarks.is_empty() {
            println!("No bookmarks yet. Use 'j add <name>' to add one.");
            return Ok(());
        }

        // 如果指定了分组，只显示该分组的书签
        if let Some(ref group) = self.group {
            let group_bookmarks = bookmarks.get_by_group(group);
            if group_bookmarks.is_empty() {
                println!("No bookmarks in group '{}'.", group);
                let groups: Vec<_> = bookmarks.list_groups().iter().map(|s| s.as_str()).collect();
                println!("Available groups: {}", groups.join(", "));
                return Ok(());
            }
            for (name, entry) in group_bookmarks {
                println!("{}/{} -> {}", group, name, entry.path);
            }
        } else {
            // 显示所有书签，按分组组织
            let groups = bookmarks.list_groups();

            // 先显示有分组的书签
            for g in &groups {
                let group_bookmarks = bookmarks.get_by_group(g);
                if !group_bookmarks.is_empty() {
                    println!("\n[{}]", g);
                    for (name, entry) in group_bookmarks {
                        println!("  {}/{} -> {}", g, name, entry.path);
                    }
                }
            }

            // 再显示没有分组的书签
            let ungrouped: Vec<_> = bookmarks
                .bookmarks
                .iter()
                .filter(|(_, entry)| entry.group.is_none())
                .collect();

            if !ungrouped.is_empty() {
                println!("\n[no group]");
                for (name, entry) in ungrouped {
                    println!("  {} -> {}", name, entry.path);
                }
            }
        }

        Ok(())
    }
}

/// 显示所有分组
pub fn list_groups(config: &Config) -> Result<(), String> {
    let bookmarks = storage::load_bookmarks(config)?;
    let groups = bookmarks.list_groups();

    if groups.is_empty() {
        println!("No groups yet. Use 'j add <name> --group <group>' to add grouped bookmarks.");
        return Ok(());
    }

    println!("Groups:");
    for group in groups {
        let count = bookmarks.get_by_group(group).len();
        println!("  {} ({} bookmark{})", group, count, if count == 1 { "" } else { "s" });
    }

    Ok(())
}
