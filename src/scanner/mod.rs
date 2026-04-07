use std::fs::read_dir;
use std::path::Path; // this is just a simple import for the Path

pub fn scan_repo() {
    println!("Hello, scanning now, ... ");
}

pub fn infer_directory(path: &Path) -> Vec<FileEntry> {
    let mut files = Vec::new();
    let entries = read_dir(path).unwrap();
    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let is_dir = path.is_dir();
                files.push(FileEntry { path, is_dir });
            }
            Err(err) => {
                println!("Error reading entry: {:?}", err);
            }
        }
    }
    files
}
