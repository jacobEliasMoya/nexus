use std::path::PathBuf;

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct ScannerData {
    pub files: Vec<FileEntry>,
}
