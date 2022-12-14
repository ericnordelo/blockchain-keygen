use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "blockchain-keygen")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate keys for the specified network
    #[command()]
    Generate {
        /// Target network for keys generation
        #[arg(short, long, default_value_t = String::from("ethereum"))]
        network: String,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
