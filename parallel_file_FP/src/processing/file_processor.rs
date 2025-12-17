use crate::threadpool::ThreadPool;
use crate::analyzer::Analyzer;
use crate::analyzer::{word_count::WordCountAnalyzer,
                      line_count::LineCountAnalyzer,
                      char_frequency::CharFrequencyAnalyzer,
                      file_size::FileSizeAnalyzer};
use crate::models::{FileAnalysis, FileStats, ProcessingError};
use crate::processing::file_discovery::discover_files;
use crate::processing::cancellation::CancellationToken;
use crate::processing::progress::{ProgressState, FileStatus};

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

pub struct FileProcessor {
    pool: ThreadPool,
    progress: Arc<Mutex<ProgressState>>,
    token: CancellationToken,
}

impl FileProcessor {
    pub fn new(workers: usize) -> Self {
        Self {
            pool: ThreadPool::new(workers),
            progress: Arc::new(Mutex::new(ProgressState::default())),
            token: CancellationToken::new(),
        }
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }

    pub fn run_on_dirs(&self, dirs: &[PathBuf]) -> Vec<FileAnalysis> {
        let files = discover_files(dirs);

        {
            let mut progress = self.progress.lock().unwrap();
            progress.total_files = files.len();

            for file in &files {
                progress.per_file.insert(
                    file.clone(),
                    crate::processing::progress::FileProgress {
                        status: FileStatus::Pending,
                        started_at: None,
                        finished_at: None,
                        last_error: None,
                    },
                );
            }
        }

        let results = Arc::new(Mutex::new(vec![]));

        for path in files {
            let analyzers: Vec<Box<dyn Analyzer>> = vec![
                Box::new(WordCountAnalyzer),
                Box::new(LineCountAnalyzer),
                Box::new(CharFrequencyAnalyzer),
            ];

            let token = self.token.clone();
            let progress = Arc::clone(&self.progress);
            let results = Arc::clone(&results);

            self.pool.execute(move || {
                let start_time = Instant::now();

                {
                    let mut prog = progress.lock().unwrap();
                    if let Some(p) = prog.per_file.get_mut(&path) {
                        p.status = FileStatus::Processing;
                        p.started_at = Some(start_time);
                    }
                }

                let content = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        let mut prog = progress.lock().unwrap();
                        if let Some(p) = prog.per_file.get_mut(&path) {
                            p.status = FileStatus::Failed;
                            p.last_error = Some(e.to_string());
                        }
                        return;
                    }
                };

                let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

                let mut stats = FileStats::default();
                let mut errors = vec![];

                let size_analyzer = FileSizeAnalyzer { size };
                size_analyzer.analyze("", &mut stats).unwrap();

                for analyzer in analyzers {
                    if token.is_cancelled() {
                        errors.push(ProcessingError::AnalyzerError {
                            analyzer: "global".to_string(),
                            message: "Cancelled during processing".into(),
                        });
                        break;
                    }

                    if let Err(msg) = analyzer.analyze(&content, &mut stats) {
                        errors.push(ProcessingError::AnalyzerError {
                            analyzer: analyzer.name().into(),
                            message: msg,
                        });
                    }
                }

                let analysis = FileAnalysis {
                    filename: path.to_string_lossy().to_string(),
                    stats,
                    errors,
                    processing_time: start_time.elapsed(),
                };

                {
                    let mut prog = progress.lock().unwrap();
                    if let Some(p) = prog.per_file.get_mut(&path) {
                        p.status = FileStatus::Done;
                        p.finished_at = Some(Instant::now());
                    }
                    prog.completed_files += 1;
                }

                {
                    results.lock().unwrap().push(analysis);
                }
            });
        }

        loop {
            let prog = self.progress.lock().unwrap();
            if prog.completed_files == prog.total_files {
                break;
            }
            drop(prog);
            std::thread::sleep(Duration::from_millis(50));
        }

        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }
}
