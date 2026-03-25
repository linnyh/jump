use crate::Config;

pub struct EditCommand;

impl EditCommand {
    pub fn execute(config: &Config) -> Result<(), String> {
        use std::process::Command as ProcCommand;

        // 获取编辑器
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        // 确保配置文件存在
        let bookmarks_path = config.bookmarks_path();
        if !bookmarks_path.exists() {
            let bookmarks_dir = bookmarks_path.parent().ok_or("Invalid config path")?;
            std::fs::create_dir_all(bookmarks_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
            std::fs::write(&bookmarks_path, r#"{"bookmarks": {}}"#)
                .map_err(|e| format!("Failed to create bookmarks file: {}", e))?;
        }

        // 打开编辑器
        let status = ProcCommand::new(&editor)
            .arg(&bookmarks_path)
            .status()
            .map_err(|e| format!("Failed to open editor: {}", e))?;

        if !status.success() {
            return Err("Editor exited with error".to_string());
        }

        // 验证 JSON 格式
        let content = std::fs::read_to_string(&bookmarks_path)
            .map_err(|e| format!("Failed to read bookmarks: {}", e))?;

        serde_json::from_str::<serde_json::Value>(&content)
            .map_err(|_| "Invalid JSON format in bookmarks file".to_string())?;

        println!("Configuration saved.");
        Ok(())
    }
}
