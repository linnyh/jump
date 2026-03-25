mod commands;
mod config;
mod core;

use clap::Parser;
use commands::{
    add_to_history, list_groups, print_session_history, AddCommand, EditCommand,
    HistCommand, InteractiveCommand, JumpCommand, ListCommand, RmCommand,
};
use config::Config;

#[derive(Parser, Debug)]
#[command(name = "j")]
#[command(version = "0.1.0")]
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
    /// Return to previous jump location (shell plugin)
    #[arg(short = 'b', long)]
    back: bool,
    /// Record current directory to session history (internal use)
    #[arg(long, hide = true)]
    record_current: bool,
    /// Current working directory (set by shell plugin)
    #[arg(long, hide = true, value_name = "DIR")]
    cwd: Option<String>,
    #[command(subcommand)]
    command: Option<Command>,
    /// Jump to directory matching pattern
    pattern: Option<String>,
}

#[derive(Parser, Debug)]
enum Command {
    /// Add a bookmark for the current directory
    Add {
        name: String,
        /// Group to organize the bookmark
        #[arg(short, long)]
        group: Option<String>,
    },
    /// Remove a bookmark
    Rm {
        name: String,
    },
    /// List all bookmarks
    List {
        /// Filter by group
        #[arg(short, long)]
        group: Option<String>,
    },
    /// Show session history and allow selection
    Hist,
    /// Show jump history
    History,
    /// Show all groups
    Groups,
    /// Show session history and allow selection
    Recent,
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

    // 记录当前目录到会话历史
    if cli.record_current {
        if let Ok(cwd) = std::env::current_dir() {
            let path = cwd.to_string_lossy().to_string();
            add_to_history(&path);
        }
        return;
    }

    // 获取 shell 传入的 cwd（如果有）
    let shell_cwd = cli.cwd.clone();

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

    match cli.command {
        Some(Command::Add { name, group }) => {
            AddCommand { name, group }.execute(&config).unwrap();
        }
        Some(Command::Rm { name }) => {
            RmCommand { name }.execute(&config).unwrap();
        }
        Some(Command::List { group }) => {
            ListCommand { group }.execute(&config).unwrap();
        }
        Some(Command::Hist) | Some(Command::History) => {
            HistCommand.execute(&config).unwrap();
        }
        Some(Command::Groups) => {
            list_groups(&config).unwrap();
        }
        Some(Command::Recent) => {
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
        }
        None => {
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
    }
}
