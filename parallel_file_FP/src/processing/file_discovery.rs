use std::fs;
use std::path::{Path, PathBuf};

pub fn discover_files(dirs: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for dir in dirs {
        collect_files_recursive(dir, &mut files);
    }

    files
}

fn collect_files_recursive(path: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();

            if p.is_dir() {
                collect_files_recursive(&p, files);
            } else if p.is_file() {
                files.push(p);
            }
        }
    }
}
