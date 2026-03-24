use clap::Parser;
use config::Config;

mod config;

#[derive(Parser, Debug)]
#[command(name = "ccd")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
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
    println!("{:?}", cli);
}
