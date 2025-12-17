use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum FileStatus {
    Pending,
    Processing,
    Done,
    Failed,
}

#[derive(Debug, Clone)]
pub struct FileProgress {
    pub status: FileStatus,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub last_error: Option<String>,
}

#[derive(Default, Debug)]
pub struct ProgressState {
    pub total_files: usize,
    pub completed_files: usize,
    pub per_file: HashMap<PathBuf, FileProgress>,
}
