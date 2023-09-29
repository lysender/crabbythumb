use std::process;
use clap::Parser;

use crate::config::{Config, Args};
use crate::run::run;

mod error;
mod config;
mod run;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run(config).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
