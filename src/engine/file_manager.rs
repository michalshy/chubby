use std::fs;
use crate::common::Result;
mod bag;
mod envi;
use {bag::Bag, envi::EnvProcessor};

pub struct FileManager {
    bag: Bag,
    env: EnvProcessor,
    exit: bool,
}

impl FileManager {
    pub fn new() -> FileManager {
        let bag = Bag::new();
        let env = EnvProcessor::new();
        let exit = false;
        FileManager {
            bag,
            env,
            exit
        }
    }

    pub fn process(&self) -> Result<()>{
        let start_point = match self.env.check_args(1) {
            Ok(start) => start,
            Err(e) => return Err(e)
        };
        self.display_first(start_point);
        while !self.exit {
        
        }    
        Ok(())
    }

    fn display_first(&self, point: String) {
        let paths = fs::read_dir(point.clone()).unwrap();
        println!("path checked: {}", point);
        for path in paths {
            if let Ok(x) = self.bag.list(path.as_ref().unwrap().path()) {
                println!("{}, size: {}", path.unwrap().path().display(), x);
            }
            else {
                println!("{}", path.unwrap().path().display());
            }
        }
    }

    fn display(&self) { 
        // let paths = fs::read_dir(point.clone()).unwrap();
        // println!("path checked: {}", point);
        // for path in paths {
        //     if let Ok(x) = self.bag.list(path.as_ref().unwrap().path()) {
        //         println!("{}, size: {}", path.unwrap().path().display(), x);
        //     }
        //     else {
        //         println!("{}", path.unwrap().path().display());
        //     }
        // }
    }
}