use crate::analyzer::Analyzer;
use crate::models::stats::FileStats;

pub struct WordCountAnalyzer;

impl Analyzer for WordCountAnalyzer {
    fn name(&self) -> &'static str {
        "word_count"
    }

    fn analyze(&self, content: &str, stats: &mut FileStats) -> Result<(), String> {
        stats.word_count = content.split_whitespace().count();
        Ok(())
    }
}
