use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// Optional name to operate on
    #[arg(short, long)]
    path: PathBuf,
}

impl Args {
    pub fn get_args() -> Self {
        Args::parse()
    }
}
