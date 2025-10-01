use std::path::PathBuf;

fn main() {
    let home = std::env::var("HOME").expect("HOME not set");
    let store_path = PathBuf::from(home).join(".powda_store.json");

    println!("HOME: {}", std::env::var("HOME").unwrap());
    println!("Store path: {:?}", store_path);
    println!("Exists: {}", store_path.exists());


    std::fs::write(&store_path, "{}").expect("failed to write");
    println!("Created file");
    println!("Now Exists: {}", store_path.exists());
}
