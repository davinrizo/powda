use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "powda")]
#[command(about = "A secure password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init{
        #[arg(short, long)]
        force: bool,
    },
    
    Unlock,
    Lock,
    Add {
        name: String,
    },
    Get {
        name: String,
    },
    List,
    Remove {
        name: String,
    },
    ChangeMaster,
}
