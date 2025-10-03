mod commands;
mod handlers;
mod ui;

use clap::Parser;
use commands::{Cli, Commands};
use handlers::PasswordHandler;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let handler = PasswordHandler::new();

    let result = match cli.command {
        Commands::Init {force} => handler.init(force).await,
        Commands::Add {name} => handler.add(name).await,
        Commands::Get {name} => handler.get(name).await,
        Commands::List => handler.list().await,
        Commands::Remove {name} => handler.remove(name).await,
        Commands::Unlock => handler.unlock().await,
        Commands::Lock => handler.lock().await,
        Commands::ChangeMaster => handler.change_master().await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
