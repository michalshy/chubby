mod searcher;
use searcher::Searcher;
use std::fs::{self};


pub struct FileManager {
    searcher: Searcher,
    start_point: String
}

impl FileManager {
    pub fn new() -> FileManager {
        let searcher = Searcher::new();
        let start_point = searcher.process_env();
        FileManager {
            searcher,
            start_point
        }
    }

    pub fn process(&self) {
        //TODO: state machine
        self.display();
    }

    fn display(&self) { 
        let paths = fs::read_dir(self.start_point.clone()).unwrap();
        println!("path checked: {}", self.start_point);
        for path in paths {
            if let Ok(x) = self.searcher.list(path.as_ref().unwrap().path()) {
                println!("{}, size: {}", path.unwrap().path().display(), x);
            }
            else {
                println!("{}", path.unwrap().path().display());
            }
        }
    }
}