mod file_manager;

pub fn run() {
    file_manager::FileManager::new().process();
}