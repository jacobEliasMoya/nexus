use std::path::pathbuf;

#[derive(debug)]
pub struct FileEntry {
    pub path: pathbuf,
}

#[derive(debug)]
pub struct ScannerData {
    pub files: vec<FileEntry>,
}
