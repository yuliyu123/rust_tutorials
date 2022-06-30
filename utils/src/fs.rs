
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}


pub fn create_dir(dir: &Path) -> io::Result<()> {
    match fs::create_dir(dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}