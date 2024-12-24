mod file_manager;

pub fn run() {
    match file_manager::FileManager::new().process() {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e)
    }
}