mod subcommands;

use clap::{Parser};
use subcommands::Commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version=true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

fn main() {
    let args = Cli::parse();
    match args.commands {
        Commands::ChangeEnv(_change_env) => {
            println!("Change");
        },
        Commands::Register(register) => {
            println!("register");
            let register_result = register.register_version();
            if register_result.is_err() {
                println!("Registration had an error");
            }
        },
        Commands::ListOfVersions(versions) => {
            let versions_result = versions.get_list();
            if versions_result.is_err() {
                println!("Error at getting list");
            }
        },
    }
}
