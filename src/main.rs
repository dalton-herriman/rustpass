mod cli;
mod entry;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("Initializing new vault...");
            // Placeholder
        }
        Commands::Add { name, username, password, notes } => {
            println!("Adding entry: {}", name);
            println!("Username: {}", username);
            println!("Password: {}", password);
            if let Some(notes) = notes {
                println!("Notes: {}", notes);
            }
        }
        Commands::Get { name } => {
            println!("Retrieving entry: {}", name);
        }
        Commands::List => {
            println!("Listing entries...");
        }
        Commands::Delete { name } => {
            println!("Deleting entry: {}", name);
        }
    }
}
