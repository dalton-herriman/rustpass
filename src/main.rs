use anyhow::Result;
use log::{info, error};
use cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod entry;
mod db;

use crate::db::db_sqlite;

fn main() -> Result<()> {
    // Initialize the logger (reads RUST_LOG env var)
    env_logger::init();

    info!("Starting rustpass CLI");

    let cli = Cli::parse();

    // Match and call handler functions, propagating errors
    match &cli.command {
        Commands::Init => handle_init(),
        Commands::Add { name, username, password, notes } => {
            handle_add(name, username, password, notes.as_ref())
        }
        Commands::Get { name } => handle_get(name),
        Commands::List => handle_list(),
        Commands::Delete { name } => handle_delete(name),
    }
}
//
// Handler function stubs returning Result<()>
//
fn handle_init() -> Result<()> {
    info!("Initializing new vault");
    println!("Initializing new vault...");
    // TODO: implement actual logic
    Ok(())
}

fn handle_add(
    name: &str,
    username: &str,
    password: &str,
    notes: Option<&String>,
) -> Result<()> {
    info!("Adding entry: {}", name);
    if name.is_empty() {
        anyhow::bail!("Entry name cannot be empty");
    }
    // TODO: add entry to vault
    println!("Adding entry: {}", name);
    println!("Username: {}", username);
    println!("Password: {}", password);
    if let Some(notes) = notes {
        println!("Notes: {}", notes);
    }
    Ok(())
}

fn handle_get(name: &str) -> Result<()> {
    info!("Retrieving entry: {}", name);
    // TODO: retrieve entry logic
    println!("Retrieving entry: {}", name);
    Ok(())
}

fn handle_list() -> Result<()> {
    info!("Listing entries");
    // TODO: list entries logic
    println!("Listing entries...");
    Ok(())
}

fn handle_delete(name: &str) -> Result<()> {
    info!("Deleting entry: {}", name);
    // TODO: delete entry logic
    println!("Deleting entry: {}", name);
    Ok(())
}
