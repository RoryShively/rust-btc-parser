use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct BlkFile {
    pub path: PathBuf,
    pub size: u64,
}