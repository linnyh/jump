mod commands;
mod config;
mod core;

use clap::Parser;
use commands::{
    add_to_history, fuzzy_match_projects, list_groups, list_project_roots,
    print_session_history, AddCommand, EditCommand, HistCommand, InteractiveCommand,
    JumpCommand, ListCommand, RmCommand,
};
use config::Config;

#[derive(Parser, Debug)]
#[command(name = "j")]
#[command(version = "0.1.1")]
#[command(before_help = "\
╭─────────────────────────────────────╮
│                                     │
│   Lightning Fast Directory Jumper   │
│                                     │
╰─────────────────────────────────────╯
")]
#[command(after_help = "\
CD-Style Commands (handled by shell plugin):
  j ..              Jump to parent directory
  j /path           Jump to absolute path
  j ../dir          Jump to relative path
  j -               Jump to previous directory
  j --back / j -b   Jump back to previous jump location

Note: These require sourcing the shell plugin (source /path/to/j/shell/j.sh)")]
struct Cli {
    /// Interactive selection mode
    #[arg(short, long)]
    interactive: bool,
    /// Open config file in editor
    #[arg(short, long)]
    edit: bool,
    /// Session history mode
    #[arg(short, long)]
    recent: bool,
    /// Add a bookmark for the current directory
    #[arg(short = 'a', long)]
    add: Option<String>,
    /// Group for the bookmark
    #[arg(long)]
    group: Option<String>,
    /// Remove a bookmark
    #[arg(short = 'd', long)]
    rm: Option<String>,
    /// List all bookmarks
    #[arg(short = 'l', long)]
    list: bool,
    /// Show all groups
    #[arg(short = 'g', long)]
    groups: bool,
    /// Show jump history
    #[arg(short = 'H', long)]
    hist: bool,
    /// Return to previous jump location (shell plugin)
    #[arg(short = 'b', long)]
    back: bool,
    /// Jump to project root (auto-detect .git, Cargo.toml, etc.)
    #[arg(short = 'R', long)]
    root: bool,
    /// Record current directory to session history (internal use)
    #[arg(long, hide = true)]
    record_current: bool,
    /// Current working directory (set by shell plugin)
    #[arg(long, hide = true, value_name = "DIR")]
    cwd: Option<String>,
    /// Jump to directory matching pattern
    pattern: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();

    if cli.interactive {
        InteractiveCommand::execute(&config).unwrap();
        return;
    }

    if cli.edit {
        EditCommand::execute(&config).unwrap();
        return;
    }

    if cli.back {
        eprintln!("j --back must be used via shell plugin: source /path/to/j/shell/j.sh");
        return;
    }

    // 获取 shell 传入的 cwd
    let shell_cwd = cli.cwd.clone();

    // -R 选项：项目根目录
    if cli.root {
        let cwd = shell_cwd
            .as_ref()
            .map(|p| std::path::PathBuf::from(p))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

        let roots = list_project_roots(&cwd);
        if let Some(pattern) = &cli.pattern {
            if let Some(root) = fuzzy_match_projects(pattern, &roots) {
                println!("{}", crate::core::jumper::generate_cd_script(&root.to_string_lossy()));
            } else {
                eprintln!("No matching project found");
                std::process::exit(1);
            }
        } else {
            if roots.is_empty() {
                println!("No project roots found");
            } else {
                println!("Project roots:\n");
                for (i, root) in roots.iter().enumerate() {
                    println!("  {}: {}", i + 1, root.display());
                }
            }
        }
        return;
    }

    // 记录当前目录到会话历史
    if cli.record_current {
        if let Ok(cwd) = std::env::current_dir() {
            let path = cwd.to_string_lossy().to_string();
            add_to_history(&path);
        }
        return;
    }

    if cli.recent {
        // -r 选项
        if let Some(pattern) = &cli.pattern {
            let result = crate::commands::recent::fuzzy_match_session_history(pattern);
            if let Some(path) = result {
                println!("{}", crate::core::jumper::generate_cd_script(&path));
            } else {
                eprintln!("No matching path in session history");
                std::process::exit(1);
            }
        } else {
            print_session_history();
        }
        return;
    }

    // -a 选项：添加书签
    if let Some(name) = &cli.add {
        AddCommand {
            name: name.clone(),
            group: cli.group.clone(),
        }
        .execute(&config)
        .unwrap();
        return;
    }

    // -d 选项：删除书签
    if let Some(name) = &cli.rm {
        RmCommand { name: name.clone() }.execute(&config).unwrap();
        return;
    }

    // -l 选项：列出书签
    if cli.list {
        ListCommand { group: cli.group.clone() }.execute(&config).unwrap();
        return;
    }

    // -g 选项：列出分组
    if cli.groups {
        list_groups(&config).unwrap();
        return;
    }

    // -H 选项：显示历史
    if cli.hist {
        HistCommand.execute(&config).unwrap();
        return;
    }

    // 无特殊选项，执行普通跳转或显示历史
    if let Some(pattern) = &cli.pattern {
        // JumpCommand 会依次匹配：本地目录 → 书签 → 会话历史
        JumpCommand {
            pattern: Some(pattern.clone()),
            cwd: shell_cwd,
        }
        .execute(&config)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
    } else {
        // 无参数，显示会话历史
        print_session_history();
    }
}
