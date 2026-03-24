mod commands;
mod config;
mod core;

use clap::Parser;
use commands::{
    add_to_history, fuzzy_match_session_history, print_session_history, AddCommand, EditCommand,
    HistCommand, InteractiveCommand, JumpCommand, ListCommand, RmCommand,
};
use config::Config;

#[derive(Parser, Debug)]
#[command(name = "j")]
#[command(version = "0.1.0")]
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
    /// Record current directory to session history (internal use)
    #[arg(long, hide = true)]
    record_current: bool,
    #[command(subcommand)]
    command: Option<Command>,
    /// Jump to directory matching pattern
    pattern: Option<String>,
}

#[derive(Parser, Debug)]
enum Command {
    Add { name: String },
    Rm { name: String },
    List,
    Hist,
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

    // 记录当前目录到会话历史
    if cli.record_current {
        if let Ok(cwd) = std::env::current_dir() {
            let path = cwd.to_string_lossy().to_string();
            add_to_history(&path);
        }
        return;
    }

    if cli.recent {
        if let Some(pattern) = &cli.pattern {
            // 模糊匹配会话历史
            if let Some(path) = fuzzy_match_session_history(pattern) {
                println!("{}", crate::core::jumper::generate_cd_script(&path));
            } else {
                eprintln!("No matching path in session history");
                std::process::exit(1);
            }
        } else {
            // 打印会话历史
            print_session_history();
        }
        return;
    }

    match cli.command {
        Some(Command::Add { name }) => {
            AddCommand { name }.execute(&config).unwrap();
        }
        Some(Command::Rm { name }) => {
            RmCommand { name }.execute(&config).unwrap();
        }
        Some(Command::List) => {
            ListCommand.execute(&config).unwrap();
        }
        Some(Command::Hist) => {
            HistCommand.execute(&config).unwrap();
        }
        Some(Command::Recent) => {
            if let Some(pattern) = &cli.pattern {
                if let Some(path) = fuzzy_match_session_history(pattern) {
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
                // 先尝试匹配会话历史
                if let Some(path) = fuzzy_match_session_history(pattern) {
                    println!("{}", crate::core::jumper::generate_cd_script(&path));
                    return;
                }

                // 再尝试书签
                let result = JumpCommand {
                    pattern: Some(pattern.clone()),
                }
                .execute(&config);

                match result {
                    Ok(_) => {}
                    Err(_) => {
                        // 书签也没有匹配，尝试交互式选择
                        InteractiveCommand::execute(&config).unwrap_or_else(|e| {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        });
                    }
                }
            } else {
                // 无参数，显示会话历史
                print_session_history();
            }
        }
    }
}
