mod bag;
mod envi;
use bag::Bag;
use envi::EnvProcessor;
use crate::common::Result;

pub struct FileManager {
    bag: Bag,
    env: EnvProcessor
}

impl FileManager {
    pub fn new() -> FileManager {
        let bag = Bag::new();
        let env = EnvProcessor::new();
        FileManager {
            bag,
            env
        }
    }

    pub fn process(&self) -> Result<()>{
        let point = match self.env.check_args() {
            Ok(start) => &start,
            Err(e) => return Err(e)
        };
        //TODO: state machine
        self.display(point);
        Ok(())
    }

    fn display(&self, point: &str) { 
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
}