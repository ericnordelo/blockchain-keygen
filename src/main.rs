mod cli;
mod commands;
mod keygen;
mod utils;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        cli::Commands::Generate { network } => commands::generate::run(network),
    }
}
