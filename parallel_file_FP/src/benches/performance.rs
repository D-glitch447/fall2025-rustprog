use criterion::{criterion_group, criterion_main, Criterion};
use parallel_file_processor::processing::file_processor::FileProcessor;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

pub fn bench_small_files(c: &mut Criterion) {
    c.bench_function("process 50 tiny files", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            for i in 0..50 {
                let p = dir.path().join(format!("f{i}.txt"));
                fs::write(p, "hello world\nthis is a test").unwrap();
            }

            let processor = FileProcessor::new(8);
            let dirs = vec![PathBuf::from(dir.path())];
            let _ = processor.run_on_dirs(&dirs);
        })
    });
}

criterion_group!(benches, bench_small_files);
criterion_main!(benches);
