use std::fmt;

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Debug, Clone)]
struct EngineError;

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Engine error occured!")
    }
}

