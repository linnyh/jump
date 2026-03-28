pub fn generate_cd_script(path: &str) -> String {
    format!("cd {}", shell_escape(path))
}

fn shell_escape(path: &str) -> String {
    if !path.contains(' ') && !path.contains('\'') && !path.contains('\\') {
        return path.to_string();
    }
    let escaped = path.replace('\'', "'\\''");
    format!("'{}'", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd_script_format() {
        let output = generate_cd_script("/test/path");
        assert!(output.contains("cd ") && output.contains("/test/path"));
    }

    #[test]
    fn test_shell_escape_simple() {
        let result = shell_escape("/simple/path");
        assert_eq!(result, "/simple/path");
    }

    #[test]
    fn test_shell_escape_with_space() {
        let result = shell_escape("/path with space");
        assert!(result.starts_with("'") && result.ends_with("'"));
    }
}
