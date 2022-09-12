use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Register {
    #[clap(long="version", short='v')]
    version: String,
    #[clap(long="path", short='p')]
    path_to_bin: String,
}