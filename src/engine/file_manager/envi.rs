use crate::common::Result;
use std::env;
pub struct EnvProcessor {
    args: Vec<String>,
}

impl EnvProcessor {
    pub fn new() -> EnvProcessor {
        let args: Vec<String> = env::args().collect();
        EnvProcessor {
            args
        }
    }
    pub fn check_args(&self, index: usize) -> Result<String> {
        match self.args.get(index) {
            Some(x) => Ok(x.to_string()),
            None => Err("No argument found".to_string())
        }
    }

}