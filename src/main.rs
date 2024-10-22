use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Calculate {
        /// The first string
        token1: String,
        /// The second string
        token2: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Calculate { token1, token2 } => {
            let res = main::levenshtein_distance_dp(&token1, &token2);
            println!("Levenshtein Distance between '{}' and '{}' is: {}", token1, token2, res);
        }
    }
}