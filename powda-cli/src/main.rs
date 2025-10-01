use clap::{Parser, Subcommand};
use rpassword::prompt_password;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "powda")]
#[command(about = "A simple password manager", long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Init,
    Add {
        name: String
    },
    Get {
        name: String
    },
    List
}

fn get_store_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".powda_store.json")
}


fn main() {
    let cli = Cli::parse();

    match cli.command{
        Commands::Init => {
            let store_path = get_store_path();
            if !store_path.exists() {
                println!("Password store already exists!");
                return;
            }

            let store: HashMap<String, String> =  HashMap::new();
            let json = serde_json::to_string(&store).unwrap();
            fs::write(store_path, json).expect("Failed to write store file");
            println!("Password store initialized!");
        }

        Commands::Add {name} => {
            let store_path = get_store_path();
            if !store_path.exists(){
                println!("Password store not initialized. Run 'powda init' first!");
                return;
            }

            let contents = fs::read_to_string(&store_path).unwrap();
            let mut store: HashMap<String, String> = serde_json::from_str(&contents).unwrap();

            let password = prompt_password("Enter Password: ").unwrap();

            store.insert(name.clone(), password);

            let json = serde_json::to_string(&store).unwrap();
            fs::write(store_path, json).unwrap();

            println!("Password for '{}' added", name);
        }

        Commands::Get { name } => {
            let store_path = get_store_path();
            if !store_path.exists() {
                println!("Password store not initialized.");
                return;
            }

            let contents = fs::read_to_string(&store_path).unwrap();
            let store: HashMap<String, String> = serde_json::from_str(&contents).unwrap();

            match store.get(&name){
                Some(password) => println!("Password: {}", password),
                None => println!("No password found for '{}'", name),
            }
        }

        Commands::List => {
        let store_path = get_store_path();
        if !store_path.exists() {
            println!("Password store not initialized");
            return;
        }

        let contents = fs::read_to_string(&store_path).unwrap();
        let store: HashMap<String, String> = serde_json:: from_str(&contents).unwrap();

        if store.is_empty() {
            println!("No passwords stored.");
            return;
        } else {
            println!("Stored passwords: ");
            for key in store.keys() {
                println!(" -- {}", key);
            }
        }
    }

    }
}
