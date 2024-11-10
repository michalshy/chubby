use manager::Manager;

pub mod manager;

fn main() {
    let start_point = r"C:\";
    Manager::new(start_point.to_string()).display();
}
