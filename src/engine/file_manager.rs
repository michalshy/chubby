use std::fs;
use crate::common::Result;
mod bag;
mod envi;
use {bag::Bag, envi::EnvProcessor};

pub struct FileManager {
    elements: Vec<(String, String)>,
    bag: Bag,
    env: EnvProcessor,
    exit: bool,
}

impl FileManager {
    pub fn new() -> FileManager {
        let bag = Bag::new();
        let env = EnvProcessor::new();
        let elements = Vec::new();
        let exit = false;
        FileManager {
            elements,
            bag,
            env,
            exit
        }
    }

    pub fn process(&mut self) -> Result<()>{
        let start_point = match self.env.check_args(1) {
            Ok(start) => start,
            Err(e) => return Err(e)
        };
        self.gather(start_point);
        self.display();
        while !self.exit {
        
        }    
        Ok(())
    }

    fn gather(&mut self, point: String) {
        let paths = fs::read_dir(point.clone()).unwrap();
        println!("path checked: {}", point);
        for path in paths {
            if let Ok(x) = self.bag.list(path.as_ref().unwrap().path()) {
                let el = 
                    path.unwrap().path().display().to_string() + 
                    ", size" +
                    x.to_string().as_str();
                self.elements.push((" ".into(), el));
            }
            else {
                let el = 
                    path.unwrap().path().display().to_string() + 
                    ", size";
                self.elements.push((" ".into(), el));
            }
        }
    }

    fn display(&self) { 
        for (pref, path) in &self.elements {
            println!("{} - {}", &pref, &path);
        }
    }
}