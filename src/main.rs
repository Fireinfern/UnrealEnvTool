mod subcommands;

use clap::{Parser};
use subcommands::Commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version=true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();
}
