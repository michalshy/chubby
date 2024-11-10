use std::{fs::{self}, io, path::PathBuf};
pub struct Traverser;

impl Traverser {
    pub fn new() -> Traverser {
        Traverser
    }

    pub fn traverse(&self, path: impl Into<PathBuf>) -> io::Result<u64> {
        fn traverse(mut dir: fs::ReadDir) -> io::Result<u64> {
            dir.try_fold(0, |acc, file| {
                let file = file?;
                let size = match file.metadata()? {
                    data if data.is_dir() => traverse(fs::read_dir(file.path())?)?,
                    data => data.len(),
                };
                Ok(acc + size)
            })
        }
        traverse(fs::read_dir(&path.into())?)
    }
}