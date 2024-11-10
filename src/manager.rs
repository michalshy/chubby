use std::{fs::{self}, io, path::PathBuf};
use traverser::Traverser;

mod traverser;

pub struct Manager {
    path: String,
    traverser: Traverser,
}

impl Manager {
    pub fn new(path: String) -> Manager {
        let traverser = Traverser::new();
        Manager {
            path,
            traverser,
        }
    }

    pub fn display(&self){
        let paths = fs::read_dir(self.path.clone()).unwrap();
        println!("path checked: {}", self.path);
        for path in paths {
            if let Ok(x) = self.traverser.traverse(path.as_ref().unwrap().path()) {
                println!("{}, size: {}", path.unwrap().path().display(), x);
            }
            else {
                println!("{}", path.unwrap().path().display());
            }
        }
    }
}