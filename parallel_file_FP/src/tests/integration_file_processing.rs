use parallel_file_processor::processing::file_processor::FileProcessor;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn processes_multiple_files_successfully() {
    let dir = tempdir().unwrap();

    // Create test file A
    let file_a = dir.path().join("a.txt");
    fs::write(&file_a, "hello world\nthis is a test").unwrap();

    // Create test file B
    let file_b = dir.path().join("b.txt");
    fs::write(&file_b, "one two three").unwrap();

    let processor = FileProcessor::new(4);

    let dirs = vec![PathBuf::from(dir.path())];
    let results = processor.run_on_dirs(&dirs);

    assert_eq!(results.len(), 2);

    // Ensure each result has stats
    for r in results {
        assert!(r.stats.word_count > 0);
        assert!(r.stats.line_count > 0);
        assert!(r.stats.size_bytes > 0);
        assert!(r.processing_time.as_millis() >= 0);
    }
}
