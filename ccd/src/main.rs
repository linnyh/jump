mod commands;
mod config;
mod core;

use clap::Parser;
use commands::{
    AddCommand, EditCommand, HistCommand, InteractiveCommand, JumpCommand, ListCommand, RmCommand,
};
use config::Config;

#[derive(Parser, Debug)]
#[command(name = "ccd")]
#[command(version = "0.1.0")]
struct Cli {
    /// Interactive selection mode
    #[arg(short, long)]
    interactive: bool,
    /// Open config file in editor
    #[arg(short, long)]
    edit: bool,
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
        None => {
            JumpCommand {
                pattern: cli.pattern,
            }
            .execute(&config)
            .unwrap();
        }
    }
}
