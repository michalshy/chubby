use std::{fs::{self}, io, path::PathBuf};
pub struct Searcher;

impl Searcher {
    pub fn new() -> Searcher {
        Searcher
    }

    pub fn process_env(&self) -> String {
        "dupa".into()
    }

    pub fn list(&self, path: impl Into<PathBuf>) -> io::Result<u64> {
        fn list(mut dir: fs::ReadDir) -> io::Result<u64> {
            dir.try_fold(0, |acc, file| {
                let file = file?;
                let size = match file.metadata()? {
                    data if data.is_dir() => list(fs::read_dir(file.path())?)?,
                    data => data.len(),
                };
                Ok(acc + size)
            })
        }
        list(fs::read_dir(&path.into())?)
    }
}