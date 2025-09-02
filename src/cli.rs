use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustpass")]
#[command(about = "A simple CLI password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vault
    Init,

    /// Add a new password entry
    Add {
        name: String,
        username: String,
        password: String,
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// Get a password entry by name
    Get {
        name: String,
    },

    /// List all entry names
    List,

    /// Delete an entry by name
    Delete {
        name: String,
    },
}
