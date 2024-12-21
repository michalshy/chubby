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
    pub fn check_args(&self) -> Result<String> {
        Ok("./".into())
    }

}