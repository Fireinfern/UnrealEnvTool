use clap::Parser;

#[derive(Parser, Debug)]
pub struct ChangeEnv {
    #[clap(long="version", short='v')]
    version: String,
}