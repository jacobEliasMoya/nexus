use std::fs::read_dir;
use std::path::Path; // this is just a simple import for the Path

pub fn scan_repo() {
    println!("Hello, scanning now");
}

pub fn infer_directory(path: &Path) {
    let entries = read_dir(path).unwrap();
    for entry in entries {
        println!("{entry:?}");
    }
}
