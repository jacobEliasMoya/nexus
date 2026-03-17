mod scanner;
use scanner::{infer_directory, scan_repo};
use std::path::Path;

fn main() {
    let current_path = Path::new("./");
    scan_repo();
    infer_directory(current_path);
}
