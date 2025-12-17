use crate::analyzer::Analyzer;
use crate::models::stats::FileStats;

pub struct FileSizeAnalyzer {
    pub size: u64,
}

impl Analyzer for FileSizeAnalyzer {
    fn name(&self) -> &'static str {
        "file_size"
    }

    fn analyze(&self, _content: &str, stats: &mut FileStats) -> Result<(), String> {
        stats.size_bytes = self.size;
        Ok(())
    }
}
