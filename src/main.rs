use std::{fs::{self, DirEntry, ReadDir}, io, path::PathBuf};

fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }
    let potential_dir = fs::read_dir(&path.into())?;

    match &potential_dir {
        ReadDir => {
            dir_size(potential_dir)
        }
        _ => Ok(0)
    }
}

fn main() -> io::Result<()>{
    let start_point = r"C:\";

    let paths = fs::read_dir(start_point).unwrap();
    println!("path checked: {}", start_point);
    for path in paths {
        if let Ok(x) = dir_size(path.as_ref().unwrap().path()) {
            println!("{}, size: {}", path.unwrap().path().display(), x);
        }
        else {
            println!("{}", path.unwrap().path().display());
        }
    }

    Ok(())
}
