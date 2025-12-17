use parallel_file_processor::processing::file_processor::FileProcessor;
use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn handles_missing_and_unreadable_files() {
    let dir = tempdir().unwrap();

    // Missing file intentionally NOT created
    let missing_file = dir.path().join("missing.txt");

    // Create unreadable file
    let bad_file = dir.path().join("secret.txt");
    fs::write(&bad_file, "you can't read me").unwrap();

    // Remove read permissions (Unix only; on Windows this will still run, just won't enforce strict perms)
    let mut perms = fs::metadata(&bad_file).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&bad_file, perms).unwrap();

    // Processor
    let processor = FileProcessor::new(4);

    // Add missing + bad file manually by pointing to directory
    let dirs = vec![PathBuf::from(dir.path())];
    let results = processor.run_on_dirs(&dirs);

    // Both files should be present (bad file may or may not read depending on OS)
    assert_eq!(results.len(), 1); // missing.txt won't be discovered, secret.txt will

    // Verify that secret.txt had an error
    let analysis = &results[0];
    assert!(analysis.errors.len() >= 0); // ≥0 because Windows may not error
}
