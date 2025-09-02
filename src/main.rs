use anyhow::Result;
use log::{info, error};
use cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod entry;

fn main() -> Result<()> {
    // Initialize the logger (reads RUST_LOG env var)
    env_logger::init();

    info!("Starting rustpass CLI");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            info!("Initializing new vault");
            println!("Initializing new vault...");
            // Placeholder
        }
        Commands::Add { name, username, password, notes } => {
            info!("Adding entry: {}", name);            
            println!("Adding entry: {}", name);
            println!("Username: {}", username);
            println!("Password: {}", password);
            if let Some(notes) = notes {
                println!("Notes: {}", notes);
            }
        }
        Commands::Get { name } => {
            info!("Retrieving entry: {}", name);
            println!("Retrieving entry: {}", name);
        }
        Commands::List => {
            info!("Listing entries");
            println!("Listing entries...");
        }
        Commands::Delete { name } => {
            info!("Deleting entry: {}", name);
            println!("Deleting entry: {}", name);
        }
    }
    Ok(())
}
